use std::u8;

use int_enum::IntEnum;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub counter: u16,
}

impl  CPU {
    pub fn new() -> Self {
        CPU {
            // Accumulator
            register_a: 0,
            register_x: 0,
            status: 0,
            counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.counter = 0;

        loop {
            let byte_code = program[self.counter as usize];
            self.counter += 1;
            
            let opcode = Opscode::try_from(byte_code);

            if opcode.is_err() {
                continue;
            }

            match opcode.unwrap() {
                Opscode::BRK_0x00 => {
                    return;
                },
                Opscode::LDA_0xA9 => {
                    // Gets param which is in counter after += (1 after function call)
                    let param = program[self.counter as usize];
                    self.counter += 1;
                    self.register_a = param;

                    self.update_flag(Flag::Zero);
                    self.update_flag(Flag::Negative);
                },
                Opscode::TAX_0xAA => {
                    self.register_x = self.register_a;

                    self.update_flag(Flag::Zero);
                    self.update_flag(Flag::Negative);
                },
                Opscode::INX_0xE8 => {
                    self.register_x = self.register_x.wrapping_add(1);

                    self.update_flag(Flag::Zero);
                    self.update_flag(Flag::Negative);
                }
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