
/// Generates an opcode and its various specific params
#[macro_export]
#[allow(unused)]
macro_rules! opcode {
    ($($name:ident $exec:expr, [$(($value:tt, $length:tt, $cycles:tt, $mode:ident)),*,]),*) => (

        #[allow(non_camel_case_types, unused, non_snake_case)]
        $(pub mod $name {
            // pub const modes: [crate::opcodes::AddressingMode; 10] = [
            //     $(crate::opcodes::AddressingMode::$mode,)*
            // ];
            $(
                #[allow(non_camel_case_types, unused, non_snake_case)]
                pub mod $mode {
                    pub const VALUE: u8 = $value;
                    pub const LEN: u8 = $length;
                    pub const CYCLES: u8 = $cycles;
                    pub fn execute(cpu: &mut crate::cpu::CPU) {
                        super::execute(cpu, crate::opcodes::AddressingMode::$mode);
                        
                        cpu.counter += (super::$mode::LEN - 1) as u16;
                    }
                }                    
            )*
            
            // Dynamically build this with a provided expr argument?
            pub fn execute(
                cpu: &mut crate::cpu::CPU, 
                mode: crate::opcodes::AddressingMode) 
            {
                $exec(cpu, mode);
            }
        })*

        // pub fn execute(cpu: &CPU, byte_code: u8) {
        //     match byte_code {
        //         $($(
        //             $name::$mode::VALUE => {
        //                 $name::$mode::execute(cpu, AddressingMode::$mode);
        //                 cpu.counter += ($name::$mode::LEN - 1) as u16;
        //             }
        //         )*)*
        //     }
        // }
    )
}

// Need to find a way to make opcode! macro able to take in
// multiple opcodes in order to also generate a pub execute function
// which can match to which opcode / params and run their specific
// execute function!
// #[macro_export]
// macro_rules! execute {
//     ($byte_code:tt) => {
//       match byte_code {

//       }
//     };
// }

opcode![
    ADC |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        todo!();
        // use crate::cpu::Flag;
        // let addr = cpu.get_operand_addr(mode);
        // let val = cpu.mem_read(addr);

        // cpu.register_y = val;
        // cpu.update_flag(Flag::Zero);
        // cpu.update_flag(Flag::Carry);
    }, [
        (0x69, 2, 2, IMMEDIATE),
        (0x65, 2, 3, ZERO_PAGE),
        (0x75, 2, 4, ZERO_PAGE_X),
        (0x6D, 3, 4, ABSOLUTE),
        (0x7D, 3, 4, ABSOLUTE_X),
        (0x79, 3, 4, ABSOLUTE_Y),
        (0x61, 2, 6, INDIRECT_X),
        (0x71, 2, 5, INDIRECT_Y),
    ],

    AND |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        todo!();
    }, [
        (0x29, 2, 2, IMMEDIATE), 
        (0x25, 2, 3, ZERO_PAGE),
        (0x35, 2, 4, ZERO_PAGE_X),
        (0x2D, 3, 4, ABSOLUTE),
        (0x3D, 3, 4, ABSOLUTE_X),
        (0x39, 3, 4, ABSOLUTE_Y),
        (0x21, 2, 6, INDIRECT_X),
        (0x31, 2, 5, INDIRECT_Y),
    ]
];

pub mod BRK_0x00 {
    pub const VALUE: u8 = 0x00;
    pub const LEN: u8 = 1;
    pub const CYCLES: u8 = 7;
}

pub mod CPY {
    use crate::cpu::{CPU, Flag};
    use super::AddressingMode;  

    pub mod IMMEDIATE_0xC0 {
        pub const VALUE: u8 = 0xC0;
        pub const LEN: u8 = 2;
        pub const CYCLES: u8 = 2;
        pub const ADD_PAGE_CROSSED: bool = false;
        pub const ADD_NEW_PAGE: bool = false;
    } 

    pub mod ZERO_PAGE_0xC4 {
        pub const VALUE: u8 = 0xC4;
        pub const LEN: u8 = 2;
        pub const CYCLES: u8 = 3;
        pub const ADD_PAGE_CROSSED: bool = false;
        pub const ADD_NEW_PAGE: bool = false;
    } 

    pub mod ABSOLUTE_0xCC {
        pub const VALUE: u8 = 0xCC;
        pub const LEN: u8 = 2;
        pub const CYCLES: u8 = 4;
        pub const ADD_PAGE_CROSSED: bool = false;
        pub const ADD_NEW_PAGE: bool = false;
    } 

    pub fn execute(cpu: &mut CPU, mode: AddressingMode) {
        let addr = cpu.get_operand_addr(mode);
        let val = cpu.mem_read(addr);

        cpu.register_y = val;
        cpu.update_flag(Flag::Zero);
        cpu.update_flag(Flag::Carry);
    }
}

/// https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
pub mod LDA {
    use crate::cpu::{CPU, Flag};

    use super::AddressingMode;    
    // One idea for splitting modes up better
    // could be better as a const hashmap if possible
    // see docs.rs/phf/latest/phf
    // pub const modes: [&u8; 7] = [
    //     &ZERO_PAGE_0xA5,
    //     &ZERO_PAGE_X_0xB5,
    //     &ABSOLUTE,
    //     &ABSOLUTE_X,
    //     &ABSOLUTE_Y,
    //     &INDIRECT_X,
    //     &INDIRECT_Y
    // ];

    pub mod IMMEDIATE_0xA9 {
        pub const VALUE: u8 = 0xA9;
        pub const LEN: u8 = 2;
        pub const CYCLES: u8 = 3;
        pub const ADD_PAGE_CROSSED: bool = false;
        pub const ADD_NEW_PAGE: bool = false;
    }
    pub mod ZERO_PAGE_0xA5 {
        pub const VALUE: u8 = 0xA5;
        pub const LEN: u8 = 2;
        pub const CYCLES: u8 = 3;
        pub const ADD_PAGE_CROSSED: bool = false;
        pub const ADD_NEW_PAGE: bool = false;
    }
    pub mod ZERO_PAGE_X_0xB5 {
        pub const VALUE: u8 = 0xB5;
        pub const LEN: u8 = 2;
        pub const CYCLES: u8 = 4;
        pub const ADD_PAGE_CROSSED: bool = false;
        pub const ADD_NEW_PAGE: bool = false;
    }
    pub mod ABSOLUTE_0xAD {
        pub const VALUE: u8 = 0xAD;
        pub const LEN: u8 = 3;
        pub const CYCLES: u8 = 4;
        pub const ADD_PAGE_CROSSED: bool = false;
        pub const ADD_NEW_PAGE: bool = false;
    }
    pub mod ABSOLUTE_X_0xBD {
        pub const VALUE: u8 = 0xBD;
        pub const LEN: u8 = 3;
        pub const CYCLES: u8 = 4;
        pub const ADD_PAGE_CROSSED: bool = true;
        pub const ADD_NEW_PAGE: bool = false;
    }
    pub mod ABSOLUTE_Y_0xB9 {
        pub const VALUE: u8 = 0xB9;
        pub const LEN: u8 = 3;
        pub const CYCLES: u8 = 4;
        pub const ADD_PAGE_CROSSED: bool = true;
        pub const ADD_NEW_PAGE: bool = false;
    }
    pub mod INDIRECT_X_0xA1 {
        pub const VALUE: u8 = 0xA1;
        pub const LEN: u8 = 2;
        pub const CYCLES: u8 = 6;
        pub const ADD_PAGE_CROSSED: bool = false;
        pub const ADD_NEW_PAGE: bool = false;
    }
    pub mod INDIRECT_Y_0xB1 {
        pub const VALUE: u8 = 0xB1;
        pub const LEN: u8 = 2;
        pub const CYCLES: u8 = 5;
        pub const ADD_PAGE_CROSSED: bool = true;
        pub const ADD_NEW_PAGE: bool = false;
    }

    /// Experimental: Returns the specific mode and command length
    /// for a given LDA opcode
    pub const fn get_mode(opcode: u8) -> (AddressingMode, u8) {
        match opcode {
            IMMEDIATE_0xA9::VALUE => {
                (AddressingMode::IMMEDIATE, IMMEDIATE_0xA9::LEN)
            }
            ZERO_PAGE_0xA5::VALUE => {
                (AddressingMode::ZERO_PAGE, ZERO_PAGE_0xA5::LEN)
            }
            ZERO_PAGE_X_0xB5::VALUE => {
                (AddressingMode::ZERO_PAGE_X, ZERO_PAGE_X_0xB5::LEN)
            }
            ABSOLUTE_0xAD::VALUE => {
                (AddressingMode::ABSOLUTE, ABSOLUTE_0xAD::LEN)
            }
            ABSOLUTE_X_0xBD::VALUE => {
                (AddressingMode::ABSOLUTE_X, ABSOLUTE_X_0xBD::LEN)
            }
            ABSOLUTE_Y_0xB9::VALUE => {
                (AddressingMode::ABSOLUTE_Y, ABSOLUTE_Y_0xB9::LEN)
            }
            INDIRECT_X_0xA1::VALUE => {
                (AddressingMode::INDIRECT_X, INDIRECT_X_0xA1::LEN)
            }
            INDIRECT_Y_0xB1::VALUE => {
                (AddressingMode::INDIRECT_Y, INDIRECT_Y_0xB1::LEN)
            }
            _ => todo!()
        }
    }

    /// Gets param which is in counter after += (1 after function call)
    /// 
    /// Experimental: Returns commands length in order to be
    /// less error prone by matching the specific command u8
    pub fn execute_combined
    (cpu: &mut CPU, opcode: u8) -> u8 {
        let (mode, length) = get_mode(opcode);
        let addr = cpu.get_operand_addr(mode);
        let val = cpu.mem_read(addr);

        cpu.register_a = val;
        cpu.update_flag(Flag::Zero);
        cpu.update_flag(Flag::Negative);

        return length;
    }

    /// Gets param which is in counter after += (1 after function call)
    pub fn execute(cpu: &mut CPU, mode: AddressingMode){
        let addr = cpu.get_operand_addr(mode);
        let val = cpu.mem_read(addr);

        cpu.register_a = val;
        cpu.update_flag(Flag::Zero);
        cpu.update_flag(Flag::Negative);
    }
}

/// https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
pub mod TAX_0xAA {
    use crate::cpu::{CPU, Flag};

    pub const VALUE: u8 = 0xAA;
    pub const LEN: u8 = 1;
    pub const CYCLES: u8 = 2;
    pub const ADD_PAGE_CROSSED: bool = false;
    pub const ADD_NEW_PAGE: bool = false;

    pub fn execute(cpu: &mut CPU) {
        cpu.register_x = cpu.register_a;

        cpu.update_flag(Flag::Zero);
        cpu.update_flag(Flag::Negative);
    }
}

/// https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
pub mod INX_0xE8 {
    use crate::cpu::{CPU, Flag};
    
    pub const VALUE: u8 = 0xE8;
    pub const LEN: u8 = 1;
    pub const CYCLES: u8 = 2;
    pub const ADD_PAGE_CROSSED: bool = false;
    pub const ADD_NEW_PAGE: bool = false;

    pub fn execute(cpu: &mut CPU) {
        cpu.register_x = cpu.register_x.wrapping_add(1);

        cpu.update_flag(Flag::Zero);
        cpu.update_flag(Flag::Negative);
    }
}

pub mod JSR_0x20 {
    use crate::cpu::{CPU, Flag};
    use super::AddressingMode::ABSOLUTE;
    
    pub const VALUE: u8 = 0x20;
    pub const LEN: u8 = 3;
    pub const CYCLES: u8 = 6;
    pub const ADD_PAGE_CROSSED: bool = false;
    pub const ADD_NEW_PAGE: bool = false;

    pub fn execute(cpu: &mut CPU) {
        todo!();
        let addr = cpu.get_operand_addr(ABSOLUTE);
        let val = cpu.mem_read(addr);

        cpu.counter = addr;
    }
}

pub mod RTS_0x60 {
    use crate::cpu::{CPU, Flag};

    pub const VALUE: u8 = 0x60;
    pub const LEN: u8 = 1;
    pub const CYCLES: u8 = 6;
    pub const ADD_PAGE_CROSSED: bool = false;
    pub const ADD_NEW_PAGE: bool = false;

    pub fn execute(cpu: &mut CPU) {
        !todo!();
        
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types, unused)]
pub enum AddressingMode {
    IMMEDIATE,
    ZERO_PAGE,
    ZERO_PAGE_X,
    ZERO_PAGE_Y,
    ABSOLUTE,
    ABSOLUTE_X,
    ABSOLUTE_Y,
    INDIRECT_X,
    INDIRECT_Y,
    NONE_ADDRESSING,
}