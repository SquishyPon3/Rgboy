use std::{ops::BitAnd, u8};

use int_enum::IntEnum;
use crate::opcodes::{*};

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub counter: u16,
    // [0x80000 .. 0xFFFF] Program ROM
    memory: [u8; PGRM_ROM_END as usize]
}

// Beginning and end of the available Program ROM memory
const PGRM_ROM_START: u16 = 0x8000;
const PGRM_ROM_END: u16 = 0xFFFF;
// Address stored within cartridge which indicates where execution begins
const PGRM_START_ADDR: u16 = 0xFFFC;

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

impl CPU {

    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            counter: 0,
            memory: [0; PGRM_ROM_END as usize]
        }
    }

    pub fn register_a_add(&mut self, data: u8) {
        let sum = 
            self.register_a as u16 + data as u16;
            todo!();
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        return u16::from_le_bytes(
            [self.mem_read(pos), self.mem_read(pos + 1)]
        );
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let bytes = data.to_le_bytes();
        self.mem_write(pos, bytes[0]);
        self.mem_write(pos + 1, bytes[1]);
    }

    // Resets the state (register and flags) and sets counter to cart start addr
    pub fn reset_interrupt(&mut self) {
        // reset method should restore the state of all registers, and initialize program_counter by the 2-byte value stored at 0xFFFC
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;
        self.counter = self.mem_read_u16(PGRM_START_ADDR)
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

    pub fn update_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Carry => todo!(),
            Flag::Zero => {
                if self.register_a == 0 {
                    self.status = self.status | 0b0000_0010;                        
                }
                else {
                    self.status = self.status & 0b1111_1101;
                }
            },
            Flag::InterruptDisable => todo!(),
            Flag::DecimalMode => todo!(),
            Flag::BreakCommand => todo!(),
            Flag::BreakCommand2 => todo!(),
            Flag::Overflow => todo!(),
            Flag::Negative => {
                if self.register_a & 0b1000_0000 != 0 {
                    self.status = self.status | 0b1000_0000;
                }
                else {
                    self.status = self.status & 0b0111_1111;
                }
            },
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, IntEnum)]
enum Opscode {
    BRK_0x00 = 0x00,
    LDA_0xA9 = 0xA9,
    TAX_0xAA = 0xAA,
    INX_0xE8 = 0xE8
}

#[repr(u8)]
pub enum Flag {
    Carry = 0b00000001,
    Zero = 0b00000010,
    InterruptDisable = 0b00000100,
    DecimalMode = 0b00001000,
    BreakCommand = 0b00010000,
    BreakCommand2 = 0b00100000,
    Overflow = 0b01000000,
    Negative = 0b10000000
}