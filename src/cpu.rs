use std::{ops::{BitAnd, BitOr, BitOrAssign}, u8};

use int_enum::IntEnum;
use crate::opcodes::{*};
use bitflags::bitflags;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: Flag,
    pub counter: u16,
    // The length of the stack STACK_START + stack_pointer to get end of stack
    pub stack_pointer: u8,
    // [0x80000 .. 0xFFFF] Program ROM
    memory: [u8; PGRM_ROM_END as usize]
}

// Beginning and end of the available Program ROM memory
const PGRM_ROM_START: u16 = 0x8000;
const PGRM_ROM_END: u16 = 0xFFFF;
// Address stored within cartridge which indicates where execution begins
const PGRM_START_ADDR: u16 = 0xFFFC;

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xFD;

#[macro_export]
macro_rules! execute {
    ($cpu:tt, $byte_code:tt, {$($opcode:ident::$mode:ident),*,}) => {
        match $byte_code {
            // Need to check specficially for BRK code
            // since it stops execution.
            0x00 => {
                return;
            }

            $(
                $opcode::$mode::VALUE => {
                    $opcode::$mode::execute($cpu);
                }
            )*
            _ => { }
        }
    }
}

pub trait Memory {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        return u16::from_le_bytes(
            [self.mem_read(pos), self.mem_read(pos + 1)]
        );
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let bytes = data.to_le_bytes();
        self.mem_write(pos, bytes[0]);
        self.mem_write(pos + 1, bytes[1]);
    }
}

impl Memory for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {

    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: Flag::from_bits_truncate(0b100100),
            counter: 0,
            stack_pointer: STACK_RESET,
            memory: [0; PGRM_ROM_END as usize]
        }
    }

    pub fn register_a_add(&mut self, data: u8) {
        let sum = self.register_a as u16 
            + data as u16
            + (match self.status.contains(Flag::Carry) {
                true => 1,
                false => 0
            }) as u16;

        let carry = sum > 0xFF;

        match carry {
            true => {self.status.insert(Flag::Carry);}
            false => {self.status.remove(Flag::Carry);}
        }

        let result = sum as u8;

        match (data ^ result) & (result ^ self.register_a) & 0x90 {
            0 => {self.status.remove(Flag::Overflow);}
            _ => {self.status.insert(Flag::Overflow);}
        }
        
        self.register_a = result;
        self.update_flag(Flag::Zero, self.register_a);
        self.update_flag(Flag::Negative, self.register_a);
    }

    // Resets the state (register and flags) and sets counter to cart start addr
    pub fn reset_interrupt(&mut self) {
        // reset method should restore the state of all registers, and initialize program_counter by the 2-byte value stored at 0xFFFC
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = Flag::from_bits_truncate(0b100100);
        self.counter = self.mem_read_u16(PGRM_START_ADDR);
        self.stack_pointer = STACK_RESET;
    }

    pub fn compare(&mut self, mode: AddressingMode, value: u8) {
        let addr = self.get_operand_addr(mode);
        let data = self.mem_read(addr);

        if data <= value {
            self.status.insert(Flag::Carry);
        } else {
            self.status.remove(Flag::Carry);  
        }
        
        self.update_flag(
            Flag::Zero, 
            value.wrapping_sub(data));
        self.update_flag(
            Flag::Negative,
            value.wrapping_sub(data));
    }
    
    pub fn logical_shift_right_a(&mut self) {

        let mut data = self.register_a;

        match data & 1 {
            1 => self.status.insert(Flag::Carry),
            _ => self.status.remove(Flag::Carry)
        }

        data = data >> 1;
        self.register_a = data;

        self.update_flag(Flag::Zero, self.register_a);
        self.update_flag(Flag::Negative, self.register_a);

        return;
    }

    pub fn logical_shift_right(&mut self, mode: AddressingMode) -> u8 {

        let addr = self.get_operand_addr(mode);
        let mut data = self.mem_read(addr);

        match data & 1 {
            1 => self.status.insert(Flag::Carry),
            _ => self.status.remove(Flag::Carry)
        }

        data = data >> 1;
        self.mem_write(addr, data);

        self.update_flag(Flag::Zero, data);
        self.update_flag(Flag::Negative, data);
        
        return data;
    }

    pub fn load_into(&mut self, mode: AddressingMode, register: Register) {
        let addr = self.get_operand_addr(mode);
        let val = self.mem_read(addr);

        match register {
            Register::A => {
                self.register_a = val;
                self.update_flag(Flag::Zero, self.register_a);
                self.update_flag(Flag::Negative, self.register_a);
            }
            Register::X => {
                self.register_x = val;
                self.update_flag(Flag::Zero, self.register_x);
                self.update_flag(Flag::Negative, self.register_x);
            },
            Register::Y => {
                self.register_y = val;
                self.update_flag(Flag::Zero, self.register_y);
                self.update_flag(Flag::Negative, self.register_y);
            },
        }
    }

    pub fn decrement_memory(&mut self, mode: AddressingMode) -> u8 {
        let addr: u16 = self.get_operand_addr(mode);
        let mut data: u8 = self.mem_read(addr);

        data = data.wrapping_sub(1);

        self.mem_write(addr, data);

        self.update_flag(Flag::Zero, data);
        self.update_flag(Flag::Negative, data);

        return data;
    }
    
    pub fn decrement_x(&mut self, mode: AddressingMode) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_flag(Flag::Zero, self.register_x);
        self.update_flag(Flag::Negative, self.register_x);
    }

    pub fn decrement_y(&mut self, mode: AddressingMode) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_flag(Flag::Zero, self.register_y);
        self.update_flag(Flag::Negative, self.register_y);
    }

    pub fn increment_memory(&mut self, mode: AddressingMode) -> u8 {
        let addr = self.get_operand_addr(mode);
        let mut data = self.mem_read(addr);

        data = data.wrapping_add(1);

        self.mem_write(addr, data);
        self.update_flag(Flag::Zero, data);
        self.update_flag(Flag::Zero, data);
        
        return data;
    }

    pub fn load(&mut self, program: Vec<u8>) {
        // load method should load a program into PRG ROM space and save the reference to the code into 0xFFFC memory cell
        self.memory[PGRM_ROM_START as usize .. (PGRM_ROM_START as usize + program.len())].copy_from_slice(&program[..]);
        // Needs to be set to addr stored in start addr const
        self.mem_write_u16(PGRM_START_ADDR, PGRM_ROM_START);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset_interrupt();
        self.run();
    }

    pub fn get_operand_addr(&self, mode: AddressingMode) -> u16 {
        use AddressingMode::*;
        match mode {
            IMMEDIATE => self.counter,

            ZERO_PAGE => self.mem_read(self.counter) as u16,

            ABSOLUTE => self.mem_read_u16(self.counter),

            ZERO_PAGE_X => {
                let pos = self.mem_read(self.counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                
                return addr;
            },

            ZERO_PAGE_Y => {
                let pos = self.mem_read(self.counter);
                let addr = pos.wrapping_add(self.register_y) as u16;

                return addr;
            },

            ABSOLUTE_X => {
                let base = self.mem_read_u16(self.counter);
                let addr = base.wrapping_add(self.register_x as u16);

                return addr;
            }

            ABSOLUTE_Y => {
                let base = self.mem_read_u16(self.counter);
                let addr = base.wrapping_add(self.register_y as u16);

                return addr;
            }

            INDIRECT_X => {
                let base = self.mem_read(self.counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);

                return (hi as u16) << 8 | (lo as u16);
            }

            INDIRECT_Y => {
                let base = self.mem_read(self.counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);

                return deref;
            }
            
            NONE_ADDRESSING => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let byte_code = self.mem_read(self.counter);
            self.counter += 1;

            execute!(self, byte_code, 
                {
                    BRK::NONE_ADDRESSING,

                    NOP::NONE_ADDRESSING,

                    ADC::IMMEDIATE, 
                    ADC::ZERO_PAGE, 
                    ADC::ZERO_PAGE_X, 
                    ADC::ABSOLUTE, 
                    ADC::ABSOLUTE_X, 
                    ADC::ABSOLUTE_Y,

                    CPY::ABSOLUTE,
                    CPY::IMMEDIATE,
                    CPY::ZERO_PAGE,

                    LDA::ABSOLUTE,
                    LDA::ABSOLUTE_X,
                    LDA::ABSOLUTE_Y,
                    LDA::IMMEDIATE,
                    LDA::INDIRECT_X,
                    LDA::INDIRECT_Y,
                    LDA::ZERO_PAGE,
                    LDA::ZERO_PAGE_X,

                    TAX_0AA::NONE_ADDRESSING,

                    INX_0xE8::NONE_ADDRESSING,
                }
            );

            //self.execute(opscode.unwrap())
        }
    }

    pub fn update_flag(&mut self, flag: Flag, register: u8) {
        
        match flag {
            Flag::Carry => todo!(),
            Flag::Zero => {
                if register == 0 {
                    self.status.insert(Flag::Zero);                       
                }
                else {
                    self.status.remove(Flag::Zero);
                }
            },
            Flag::InterruptDisable => todo!(),
            Flag::DecimalMode => todo!(),
            Flag::BreakCommand => todo!(),
            Flag::BreakCommand2 => todo!(),
            Flag::Overflow => todo!(),
            Flag::Negative => {
                if register >> 1 == 1 {
                    self.status.insert(Flag::Negative);
                }
                else {
                    self.status.remove(Flag::Negative);
                }
            },
            _ => todo!()
        }
    }
}

bitflags! {
    #[derive(PartialEq, Eq)]
    #[derive(Clone, Copy)]
    pub struct Flag: u8 {
        const Carry = 0b0000_0001;
        const Zero = 0b0000_0010;
        const InterruptDisable = 0b0000_0100;
        const DecimalMode = 0b0000_1000;
        const BreakCommand = 0b000_10000;
        const BreakCommand2 = 0b0010_0000;
        const Overflow = 0b0100_0000;
        const Negative = 0b1000_0000;
    }
}

pub enum Register {
    A,
    X,
    Y       
}