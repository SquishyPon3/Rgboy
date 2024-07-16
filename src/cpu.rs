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
    pub counter_state: u16,
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
                // unsafe {
                //     use std::fs::File;
                //     use std::io::prelude::*;
                //     let mut file = File::create("history.txt").unwrap();
                //     file.write(crate::opcodes::HISTORY.as_bytes()).unwrap();
                // }                
                return;
            }

            $(
                $opcode::$mode::VALUE => {
                    $opcode::$mode::execute($cpu);
                }
            )*
            _ => { 
                use std::fs::File;
                use std::io::prelude::*;
                let mut file = File::create("memory.txt").unwrap();
                let mem_dump = format!("{:#04X?}", &$cpu.memory);
                file.write(mem_dump.as_bytes()).unwrap();
                panic!("No opcode for {:#04X?}", $byte_code) 
            }
        }
    }
}

const SNAKE_GAME: [u8; 16 * 19 + 5] = [
    0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02, 0x85,
    0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9, 0x0f, 0x85,
    0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe,
    0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3, 
    0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9,
    0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60,
    0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
    0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02,
    0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06,
    0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07,
    0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
    0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
    0x35, 0x07, 0x60, 0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02,
    0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
    0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
    0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xb0,
    0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29,
    0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60,
    0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea,
    0xea, 0xca, 0xd0, 0xfb, 0x60
];

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

    pub fn load_snake(&mut self) {
        self.memory[0x0600..(0x0600 + SNAKE_GAME.len())].copy_from_slice(&SNAKE_GAME[..]);
        self.mem_write_u16(0xFFFC, 0x0600)
    }

    pub fn run_snake_with_callback<F>(&mut self, mut call_back: F)
    where F: FnMut(&mut CPU),
    {
        use crate::exec_opcodes;
        //let mut execution: Vec<u8> = vec![];
        
        loop {
            let byte_code = self.mem_read(self.counter);
            self.counter += 1;
            self.counter_state = self.counter;

            exec_opcodes!(self, byte_code);

            //execution.push(byte_code);            

            call_back(self);
        }
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
            counter_state: 0,
            stack_pointer: STACK_RESET,
            memory: [0; PGRM_ROM_END as usize]
        }
    }

    pub fn stack_pull(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        return self.mem_read(
            STACK as u16 + self.stack_pointer as u16)
    }

    pub fn stack_pull_u16(&mut self) -> u16 {
        let lo = self.stack_pull() as u16;
        let hi = self.stack_pull() as u16;

        return hi << 8 | lo
    }

    pub fn stack_push(&mut self, data: u8) {
        self.mem_write(
            STACK as u16 + self.stack_pointer as u16,
            data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn stack_push_u16(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & u8::MAX as u16) as u8;

        self.stack_push(hi);
        self.stack_push(lo);
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
        self.counter_state = self.counter;
        self.stack_pointer = STACK_RESET;
    }

    pub fn compare(&mut self, mode: AddressingMode, mut value: u8) {
        let addr = self.get_operand_addr(mode);
        let data = self.mem_read(addr);

        if data <= value {
            self.status.insert(Flag::Carry);
        } else {
            self.status.remove(Flag::Carry);  
        }

        value = value.wrapping_sub(data);
        
        self.update_flag(Flag::Zero,value);
        self.update_flag(Flag::Negative,value);
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

    pub fn rotate_left_a(&mut self) {
        let mut data = self.register_a;
        let had_carry = self.status.contains(Flag::Carry);

        match data >> 7 {
            1 => self.status.insert(Flag::Carry),
            _ => self.status.remove(Flag::Carry)
        }

        data <<= 1;

        if had_carry {
            data = data | 0b1000_0000;
        }

        self.register_a = data;
        self.update_flag(Flag::Zero, data);
        self.update_flag(Flag::Negative, data);
    }

    pub fn rotate_left(&mut self, mode: AddressingMode) -> u8 {
        let addr = self.get_operand_addr(mode);
        let mut data = self.mem_read(addr);
        let had_carry = self.status.contains(Flag::Carry);

        match data >> 7 {
            1 => self.status.insert(Flag::Carry),
            _ => self.status.remove(Flag::Carry)
        }

        data <<= 1;

        if had_carry {
            data |= 1;
        }

        self.mem_write(addr, data);
        self.update_flag(Flag::Negative, data);

        return data;
    }

    pub fn rotate_right_a(&mut self) {
        let mut data = self.register_a;
        let had_carry = self.status.contains(Flag::Carry);

        match data << 7 {
            1 => self.status.insert(Flag::Carry),
            _ => self.status.remove(Flag::Carry)
        }

        data >>= 1;

        if had_carry {
            data = data | 0b1000_0000;
        }

        self.register_a = data;
        self.update_flag(Flag::Zero, data);
        self.update_flag(Flag::Negative, data);
    }

    pub fn rotate_right(&mut self, mode: AddressingMode) -> u8 {
        let addr = self.get_operand_addr(mode);
        let mut data = self.mem_read(addr);
        let had_carry = self.status.contains(Flag::Carry);

        match data << 7 {
            1 => self.status.insert(Flag::Carry),
            _ => self.status.remove(Flag::Carry)
        }

        data >>= 1;

        if had_carry {
            data = data | 0b1000_0000;
        }

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

    pub fn branch(&mut self, case: bool) {
        if case {
            let jump: i8 = self.mem_read(self.counter) as i8;
            let addr = self
                .counter
                .wrapping_add(1)
                .wrapping_add(jump as u16);

            self.counter = addr;
        }
    }

    pub fn jump(&mut self, mode: AddressingMode) {

        let addr = self.mem_read_u16(self.counter);
        
        match mode {

            AddressingMode::ABSOLUTE => {
                self.counter = addr;
            }
            AddressingMode::NONE_ADDRESSING => {
                self.counter = addr;

                // let indirect_ref = self.mem_read_u16(mem_address);
                // 6502 bug mode with with page boundary:
                // if address $3000 contains $40, $30FF contains $80, 
                // and $3100 contains $50, the result of JMP ($30FF) 
                // will be a transfer of control to $4080 rather than
                // $5080 as you intended i.e. the 6502 took the low byte 
                // of the address from $30FF and the high byte from $3000

                let indirect_ref = match addr & 0x00FF {
                    0x00FF => {
                        let lo = self.mem_read(addr);
                        let hi = self.mem_read(addr & 0xFF00);
                        (hi as u16) << 8 | (lo as u16)
                    }
                    _ => self.mem_read_u16(addr)
                };

                self.counter = indirect_ref;
            }
            _ => panic!("JUMP PROVIDED INCORRECT ADDRESSING MODE")
        }
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
        use crate::exec_opcodes;

        loop {
            let byte_code = self.mem_read(self.counter);
            self.counter += 1;

            exec_opcodes!(self, byte_code);
        }
    }

    pub fn update_flag(&mut self, flag: Flag, register: u8) {
        
        match flag {
            Flag::Zero => {
                if register == 0 {
                    self.status.insert(Flag::Zero);                       
                }
                else {
                    self.status.remove(Flag::Zero);
                }
            },
            Flag::Negative => {
                if register >> 7 == 1 {
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

// Temporarily moved this setup to a macro in order
// to throw this into the snake impl for CPU without
// having to paste the entire thing in...
#[macro_export]
#[allow(unused)]
macro_rules! exec_opcodes {
    ($cpu:tt, $byte_code:tt) => {
        execute!(
            $cpu, 
            $byte_code, 
            {
                /* Special */
                BRK::NONE_ADDRESSING,

                NOP::NONE_ADDRESSING,

                /* Arithmetic */
                ADC::IMMEDIATE, 
                ADC::ZERO_PAGE, 
                ADC::ZERO_PAGE_X, 
                ADC::ABSOLUTE, 
                ADC::ABSOLUTE_X, 
                ADC::ABSOLUTE_Y,

                SBC::IMMEDIATE,
                SBC::ZERO_PAGE,
                SBC::ZERO_PAGE_X,
                SBC::ABSOLUTE,
                SBC::ABSOLUTE_X,
                SBC::ABSOLUTE_Y,
                SBC::INDIRECT_X,
                SBC::INDIRECT_Y,
                
                AND::IMMEDIATE, 
                AND::ZERO_PAGE, 
                AND::ZERO_PAGE_X, 
                AND::ABSOLUTE, 
                AND::ABSOLUTE_X, 
                AND::ABSOLUTE_Y,
                AND::INDIRECT_X, 
                AND::INDIRECT_Y,

                EOR::IMMEDIATE, 
                EOR::ZERO_PAGE, 
                EOR::ZERO_PAGE_X, 
                EOR::ABSOLUTE, 
                EOR::ABSOLUTE_X, 
                EOR::ABSOLUTE_Y,
                EOR::INDIRECT_X, 
                EOR::INDIRECT_Y,

                ORA::IMMEDIATE, 
                ORA::ZERO_PAGE, 
                ORA::ZERO_PAGE_X, 
                ORA::ABSOLUTE, 
                ORA::ABSOLUTE_X, 
                ORA::ABSOLUTE_Y,
                ORA::INDIRECT_X, 
                ORA::INDIRECT_Y,

                DEC::ZERO_PAGE, 
                DEC::ZERO_PAGE_X, 
                DEC::ABSOLUTE, 
                DEC::ABSOLUTE_X, 

                /* Shifts */
                ASL::NONE_ADDRESSING, // Accumulator
                ASL::ZERO_PAGE, 
                ASL::ZERO_PAGE_X, 
                ASL::ABSOLUTE, 
                ASL::ABSOLUTE_X, 

                ROL::NONE_ADDRESSING, // Accumulator
                ROL::ZERO_PAGE, 
                ROL::ZERO_PAGE_X, 
                ROL::ABSOLUTE, 
                ROL::ABSOLUTE_X, 

                ROR::NONE_ADDRESSING, // Accumulator
                ROR::ZERO_PAGE, 
                ROR::ZERO_PAGE_X, 
                ROR::ABSOLUTE, 
                ROR::ABSOLUTE_X, 

                INC::ZERO_PAGE,
                INC::ZERO_PAGE_X,
                INC::ABSOLUTE,
                INC::ABSOLUTE_X,

                DEM::ZERO_PAGE, 
                DEM::ZERO_PAGE_X, 
                DEM::ABSOLUTE, 
                DEM::ABSOLUTE_X, 

                DEX::NONE_ADDRESSING,
                
                DEY::NONE_ADDRESSING,

                CMP::IMMEDIATE, 
                CMP::ZERO_PAGE, 
                CMP::ZERO_PAGE_X, 
                CMP::ABSOLUTE, 
                CMP::ABSOLUTE_X, 
                CMP::ABSOLUTE_Y,
                CMP::INDIRECT_X, 
                CMP::INDIRECT_Y,

                CPX::IMMEDIATE, 
                CPX::ZERO_PAGE, 
                CPX::ABSOLUTE, 
                
                CPY::IMMEDIATE,
                CPY::ZERO_PAGE,
                CPY::ABSOLUTE,

                LSR::NONE_ADDRESSING, // Accumulator 
                LSR::ZERO_PAGE, 
                LSR::ZERO_PAGE_X, 
                LSR::ABSOLUTE, 
                LSR::ABSOLUTE_X, 

                /* Branching */
                JMP::ABSOLUTE,
                JMP::NONE_ADDRESSING,

                RTS::NONE_ADDRESSING,

                RTI::NONE_ADDRESSING,

                JSR::ABSOLUTE,

                BCC::NONE_ADDRESSING,

                BCS::NONE_ADDRESSING,

                BEQ::NONE_ADDRESSING,

                BMI::NONE_ADDRESSING,

                BNE::NONE_ADDRESSING,

                BPL::NONE_ADDRESSING,

                BVC::NONE_ADDRESSING,

                BVS::NONE_ADDRESSING,

                BIT::ZERO_PAGE,
                BIT::ABSOLUTE,

                /* Stores & Loads */
                LDA::IMMEDIATE,
                LDA::ZERO_PAGE,
                LDA::ZERO_PAGE_X,
                LDA::ABSOLUTE,
                LDA::ABSOLUTE_X,
                LDA::ABSOLUTE_Y,
                LDA::INDIRECT_X,
                LDA::INDIRECT_Y,

                LDX::IMMEDIATE,
                LDX::ZERO_PAGE,
                LDX::ZERO_PAGE_Y,
                LDX::ABSOLUTE,
                LDX::ABSOLUTE_Y,

                LDY::IMMEDIATE,
                LDY::ZERO_PAGE,
                LDY::ZERO_PAGE_X,
                LDY::ABSOLUTE,
                LDY::ABSOLUTE_X,

                STA::ZERO_PAGE,
                STA::ZERO_PAGE_X,
                STA::ABSOLUTE,
                STA::ABSOLUTE_X,
                STA::ABSOLUTE_Y,
                STA::INDIRECT_X,
                STA::INDIRECT_Y,

                STX::ZERO_PAGE,
                STX::ZERO_PAGE_X,
                STX::ABSOLUTE,

                STY::ZERO_PAGE,
                STY::ZERO_PAGE_X,
                STY::ABSOLUTE,

                /* Flags Clear */
                CLD::NONE_ADDRESSING,

                CLI::NONE_ADDRESSING,

                CLV::NONE_ADDRESSING,

                CLC::NONE_ADDRESSING,

                TAX::NONE_ADDRESSING,

                TAY::NONE_ADDRESSING,

                INX::NONE_ADDRESSING,

                INY::NONE_ADDRESSING,

                SEC::NONE_ADDRESSING,

                SEI::NONE_ADDRESSING,

                SED::NONE_ADDRESSING,

                TSX::NONE_ADDRESSING,

                TXA::NONE_ADDRESSING,

                TXS::NONE_ADDRESSING,

                TYA::NONE_ADDRESSING,

                /* Stack */
                PHA::NONE_ADDRESSING,

                PHP::NONE_ADDRESSING,

                PLA::NONE_ADDRESSING,

                PLP::NONE_ADDRESSING,
            }
        );
    }
}