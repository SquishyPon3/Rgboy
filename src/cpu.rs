use std::u8;

use int_enum::IntEnum;
use crate::opcodes::OPCODE_LDA;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
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

impl  CPU {

    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            counter: 0,
            memory: [0; PGRM_ROM_END as usize]
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
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
    fn reset_interrupt(&mut self) {
        // reset method should restore the state of all registers, and initialize program_counter by the 2-byte value stored at 0xFFFC
        self.register_a = 0;
        self.register_x = 0;
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

    pub fn run(&mut self) {
        loop {
            let byte_code = self.mem_read(self.counter);
            self.counter += 1;

            let opscode = Opscode::try_from(byte_code);

            if opscode.is_err() {
                return;
            }

            // let a = OPCODE_LDA.IMMEDIATE_0xA9;
            match byte_code {
                val if OPCODE_LDA.IMMEDIATE_0xA9 == byte_code => {
                    
                }

                _ => todo!(),
            }

            self.execute(opscode.unwrap())
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

    fn update_flag(&mut self, flag: Flag) {
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

enum Flag {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode,
    BreakCommand,
    Overflow,
    Negative
}