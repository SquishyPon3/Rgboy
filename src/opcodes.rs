
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
    )
}

// macro_rules! set_bitflags {

// }

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
    BRK |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        return;
    }, [
        (0x00, 1, 7, NONE_ADDRESSING),
    ],

    NOP |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        return;
    }, [
        (0xEA, 1, 2, NONE_ADDRESSING),
    ],

    /* Arithmetic */
    ADC |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{CPU, Memory};

        let addr = cpu.get_operand_addr(mode);
        let value = cpu.mem_read(addr);
        
        cpu.register_a_add(value);
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
        use crate::cpu::Memory;

        let addr = cpu.get_operand_addr(mode);
        let val = cpu.mem_read(addr);

        cpu.register_a &= val;
        todo!("This does not do any evaluation yet, or set any flags...");
    }, [
        (0x29, 2, 2, IMMEDIATE), 
        (0x25, 2, 3, ZERO_PAGE),
        (0x35, 2, 4, ZERO_PAGE_X),
        (0x2D, 3, 4, ABSOLUTE),
        (0x3D, 3, 4, ABSOLUTE_X),
        (0x39, 3, 4, ABSOLUTE_Y),
        (0x21, 2, 6, INDIRECT_X),
        (0x31, 2, 5, INDIRECT_Y),
    ],

    DEC |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.decrement_memory(mode);
    }, [
        (0x25, 2, 5, ZERO_PAGE),
        (0x35, 2, 6, ZERO_PAGE_X),
        (0x2D, 3, 6, ABSOLUTE),
        (0x29, 3, 7, ABSOLUTE_X), 
    ],

    /* Shifts */
    CMP |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory, Flag};
        cpu.compare(mode, cpu.register_a);
    }, [
        (0xC9, 2, 2, IMMEDIATE),
        (0xC5, 2, 3, ZERO_PAGE),
        (0xD5, 2, 4, ZERO_PAGE_X),
        (0xCD, 2, 4, ABSOLUTE),
        (0xDD, 3, 4, ABSOLUTE_X), // +1 if page crossed
        (0xD9, 3, 4, ABSOLUTE_Y), // +1 if page crossed
        (0xC1, 2, 6, INDIRECT_X),
        (0xD1, 2, 5, INDIRECT_Y), // +1 if page crossed
    ],

    CPX |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory, Flag};

        cpu.compare(mode, cpu.register_x);
    }, [
        (0xE0, 2, 2, IMMEDIATE),
        (0xE4, 2, 3, ZERO_PAGE),
        (0xEC, 3, 4, ABSOLUTE),
    ],

    CPY |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory, Flag};

        cpu.compare(mode, cpu.register_y)
    }, [
        (0xC0, 2, 2, IMMEDIATE),
        (0xC4, 2, 3, ZERO_PAGE),
        (0xCC, 3, 4, ABSOLUTE),
    ],

    LSR |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        
        // A,C,Z,N = A/2 or M,C,Z,N = M/2

        // Each of the bits in A or M is shift one place to the right. 
        // The bit that was in bit 0 is shifted into the carry flag. 
        // Bit 7 is set to zero.

        if matches!(mode, super::AddressingMode::NONE_ADDRESSING) {
            cpu.logical_shift_right_a();
            return;
        }

        _ = cpu.logical_shift_right(mode);
    }, [
        (0x4A, 1, 2, NONE_ADDRESSING), // Addressing Accumulator
        (0x46, 2, 5, ZERO_PAGE),
        (0x56, 2, 6, ZERO_PAGE_X),
        (0x4E, 3, 6, ABSOLUTE),
        (0x5E, 3, 7, ABSOLUTE_X),
    ],

    /* Stores & Loads */
    LDA |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Register};

        cpu.load_into(mode, Register::A);
    }, [
        (0xA9, 2, 2, IMMEDIATE),
        (0xA5, 2, 3, ZERO_PAGE),
        (0xB5, 2, 4, ZERO_PAGE_X),
        (0xAD, 3, 4, ABSOLUTE), // +1 if page crossed
        (0xBD, 3, 4, ABSOLUTE_X),
        (0xB9, 3, 4, ABSOLUTE_Y), // +1 if page crossed 
        (0xA1, 2, 6, INDIRECT_X),
        (0xB1, 2, 5, INDIRECT_Y), // +1 if page crossed
    ],

    LDX |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Register};

        cpu.load_into(mode, Register::X);
    }, [
        (0xA2, 2, 2, IMMEDIATE),
        (0xA6, 2, 3, ZERO_PAGE),
        (0xB6, 2, 4, ZERO_PAGE_Y),
        (0xAE, 3, 4, ABSOLUTE),
        (0xBE, 3, 4, ABSOLUTE_Y), // +1 if page crossed
    ],

    LDY |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Register};

        cpu.load_into(mode, Register::Y);
    }, [
        (0xA0, 2, 2, IMMEDIATE),
        (0xA4, 2, 3, ZERO_PAGE),
        (0xB4, 2, 4, ZERO_PAGE_X),
        (0xAC, 3, 4, ABSOLUTE),
        (0xBC, 3, 4, ABSOLUTE_X), // +1 if page crossed
    ],

    STA |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory};

        let addr = cpu.get_operand_addr(mode);
        cpu.mem_write(addr, cpu.register_a);
    }, [
        (0x85, 2, 3, ZERO_PAGE),
        (0x95, 2, 4, ZERO_PAGE_X),
        (0x8D, 3, 4, ABSOLUTE),
        (0x9D, 3, 5, ABSOLUTE_X),
        (0x99, 3, 5, ABSOLUTE_Y),
        (0x81, 2, 6, INDIRECT_X),
        (0x91, 2, 6, INDIRECT_Y),
    ],

    STX |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory};

        let addr = cpu.get_operand_addr(mode);
        cpu.mem_write(addr, cpu.register_x);
    }, [
        (0x86, 2, 3, ZERO_PAGE),
        (0x96, 2, 4, ZERO_PAGE_X),
        (0x8E, 3, 4, ABSOLUTE),
    ],

    STY |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory};

        let addr = cpu.get_operand_addr(mode);
        cpu.mem_write(addr, cpu.register_y);
    }, [
        (0x84, 2, 3, ZERO_PAGE),
        (0x94, 2, 4, ZERO_PAGE_X),
        (0x8C, 3, 4, ABSOLUTE),
    ],

    /* Flags clear */
    // https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
    TAX_0AA |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.register_x = cpu.register_a;
    
        cpu.update_flag(crate::cpu::Flag::Zero, cpu.register_x);
        cpu.update_flag(crate::cpu::Flag::Negative, cpu.register_x);
    }, [
        (0xAA, 1, 2, NONE_ADDRESSING),
    ],

    TAY |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.register_y = cpu.register_a;
    
        cpu.update_flag(crate::cpu::Flag::Zero, cpu.register_y);
        cpu.update_flag(crate::cpu::Flag::Negative, cpu.register_y);
    }, [
        (0xA8, 1, 2, NONE_ADDRESSING),
    ],

    // https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
    INX_0xE8 |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.register_x = cpu.register_x.wrapping_add(1);

        cpu.update_flag(crate::cpu::Flag::Zero, cpu.register_x);
        cpu.update_flag(crate::cpu::Flag::Negative, cpu.register_x);
    }, [
        (0xE8, 1, 2, NONE_ADDRESSING),
    ],

    INY |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.register_y = cpu.register_y.wrapping_add(1);

        cpu.update_flag(crate::cpu::Flag::Zero, cpu.register_y);
        cpu.update_flag(crate::cpu::Flag::Negative, cpu.register_y);
    }, [
        (0xE8, 1, 2, NONE_ADDRESSING),
    ],

    CLD |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.remove(crate::cpu::Flag::DecimalMode);  
    }, [
        (0xD8, 1, 2, NONE_ADDRESSING),
    ],

    CLI |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.remove(crate::cpu::Flag::InterruptDisable);  
    }, [
        (0x58, 1, 2, NONE_ADDRESSING),
    ],

    CLV |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.remove(crate::cpu::Flag::Overflow);  
    }, [
        (0xB8, 1, 2, NONE_ADDRESSING),
    ],

    CLC |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.remove(crate::cpu::Flag::Carry);  
    }, [
        (0x18, 1, 2, NONE_ADDRESSING),
    ],

    SEC |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.remove(crate::cpu::Flag::Overflow);  
    }, [
        (0x38, 1, 2, NONE_ADDRESSING),
    ]

    /* Stack */
];

pub mod JSR_0x20 {
    use crate::cpu::{CPU, Flag};
    use super::AddressingMode::ABSOLUTE;
    
    pub const VALUE: u8 = 0x20;
    pub const LEN: u8 = 3;
    pub const CYCLES: u8 = 6;
    pub const ADD_PAGE_CROSSED: bool = false;
    pub const ADD_NEW_PAGE: bool = false;

    pub fn execute(cpu: &mut CPU) {

        // let addr = cpu.get_operand_addr(ABSOLUTE);
        // let val = cpu.mem_read(addr);

        // cpu.counter = addr;
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
        return;        
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