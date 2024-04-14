use std::u8;

use int_enum::IntEnum;
use crate::opcodes::{self, *};

use self::LDA::INDIRECT_X_0xA1;

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
            Immediate => self.counter,

            ZeroPage => self.mem_read(self.counter) as u16,

            Absolute => self.mem_read_u16(self.counter),

            ZeroPage_X => {
                let pos = self.mem_read(self.counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                
                return addr;
            },

            ZeroPage_Y => {
                let pos = self.mem_read(self.counter);
                let addr = pos.wrapping_add(self.register_y) as u16;

                return addr;
            },

            Absolute_X => {
                let base = self.mem_read_u16(self.counter);
                let addr = base.wrapping_add(self.register_x as u16);

                return addr;
            }

            Absolute_Y => {
                let base = self.mem_read_u16(self.counter);
                let addr = base.wrapping_add(self.register_y as u16);

                return addr;
            }

            Indirect_X => {
                let base = self.mem_read(self.counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);

                return (hi as u16) << 8 | (lo as u16);
            }

            Indirect_Y => {
                let base = self.mem_read(self.counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);

                return deref;
            }
            
            NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let byte_code = self.mem_read(self.counter);
            self.counter += 1;

            //let opscode = Opscode::try_from(byte_code);

            // if opscode.is_err() {
            //     return;
            // }

            match byte_code {
                BRK_0x00::VALUE => {
                    self.counter += (BRK_0x00::LEN - 1) as u16;
                    return;
                }

                CPY::ABSOLUTE_0xCC::VALUE => {
                    CPY::execute(self, AddressingMode::Absolute);
                    self.counter += (CPY::ABSOLUTE_0xCC::LEN - 1) as u16;
                }
                
                // Experimental combined execute
                // which requires another whole match statement
                // in order to avoid errors.
                // I intend to look into macros to solve this problem
                // instead.
                LDA::IMMEDIATE_0xA9::VALUE 
                | LDA::ZERO_PAGE_0xA5::VALUE
                | LDA::ZERO_PAGE_X_0xB5::VALUE
                | LDA::ABSOLUTE_0xAD::VALUE
                | LDA::ABSOLUTE_X_0xBD::VALUE
                | LDA::ABSOLUTE_Y_0xB9::VALUE
                | LDA::INDIRECT_X_0xA1::VALUE
                | LDA::INDIRECT_Y_0xB1::VALUE => {                    
                    self.counter += (LDA::execute_combined(self, byte_code) - 1) as u16;
                }

                LDA::IMMEDIATE_0xA9::VALUE => {
                    LDA::execute(self, AddressingMode::Immediate);
                    self.counter += (LDA::IMMEDIATE_0xA9::LEN - 1) as u16;                    
                }
                LDA::ZERO_PAGE_0xA5::VALUE => {
                    LDA::execute(self, AddressingMode::ZeroPage);
                    self.counter += (LDA::ZERO_PAGE_0xA5::LEN - 1) as u16;
                }
                LDA::ZERO_PAGE_X_0xB5::VALUE => {
                    LDA::execute(self, AddressingMode::ZeroPage_X);
                    self.counter += (LDA::ZERO_PAGE_X_0xB5::LEN - 1) as u16;
                }
                LDA::ABSOLUTE_0xAD::VALUE => {
                    LDA::execute(self, AddressingMode::Absolute);
                    self.counter += (LDA::ABSOLUTE_0xAD::LEN - 1) as u16;
                }
                LDA::ABSOLUTE_X_0xBD::VALUE => {
                    LDA::execute(self, AddressingMode::Absolute_X);
                    self.counter += (LDA::ABSOLUTE_X_0xBD::LEN - 1) as u16;
                }
                LDA::ABSOLUTE_Y_0xB9::VALUE => {
                    LDA::execute(self, AddressingMode::Absolute_Y);
                    self.counter += (LDA::ABSOLUTE_Y_0xB9::LEN - 1) as u16;
                }
                LDA::INDIRECT_X_0xA1::VALUE => {
                    LDA::execute(self, AddressingMode::Indirect_X);
                    self.counter += (LDA::INDIRECT_X_0xA1::LEN - 1) as u16;
                }
                LDA::INDIRECT_Y_0xB1::VALUE => {
                    LDA::execute(self, AddressingMode::Indirect_Y);
                    self.counter += (LDA::INDIRECT_Y_0xB1::LEN - 1) as u16;
                }

                TAX_0xAA::VALUE => {
                    TAX_0xAA::execute(self);
                    self.counter += (TAX_0xAA::LEN - 1) as u16;
                }

                INX_0xE8::VALUE => {
                    INX_0xE8::execute(self);
                    self.counter += (INX_0xE8::LEN - 1) as u16;
                }

                _ => todo!(),
            }

            //self.execute(opscode.unwrap())
        }
    }

    fn execute(&mut self, opscode: Opscode) {
            
        match opscode {
            Opscode::BRK_0x00 => {
                // https://www.nesdev.org/obelisk-6502-guide/reference.html#BRK
                return;
            },
            Opscode::LDA_0xA9 => {
                // https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
                // Gets param which is in counter after += (1 after function call)
                let param = self.mem_read(self.counter);
                self.counter += 1;
                self.register_a = param;

                self.update_flag(Flag::Zero);
                self.update_flag(Flag::Negative);
            },
            Opscode::TAX_0xAA => {
                // https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
                self.register_x = self.register_a;

                self.update_flag(Flag::Zero);
                self.update_flag(Flag::Negative);
            },
            Opscode::INX_0xE8 => {
                // https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
                self.register_x = self.register_x.wrapping_add(1);

                self.update_flag(Flag::Zero);
                self.update_flag(Flag::Negative);
            }
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

pub enum Flag {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode,
    BreakCommand,
    Overflow,
    Negative
}