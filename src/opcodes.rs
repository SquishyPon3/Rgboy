
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
    // Special case
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

    // Add with carry
    // A,Z,C,N = A + M + C
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

    // Logical And
    // A,Z,N = A & M
    AND |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory, Flag};

        let addr = cpu.get_operand_addr(mode);
        let data = cpu.mem_read(addr);

        cpu.register_a &= data;

        cpu.update_flag(Flag::Zero, cpu.register_a);
        cpu.update_flag(Flag::Negative, cpu.register_a);
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

    // Exclusive Or
    // A,Z,N = A^M
    EOR |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory, Flag};

        let addr = cpu.get_operand_addr(mode);
        let data = cpu.mem_read(addr);

        cpu.register_a ^= data;

        cpu.update_flag(Flag::Zero, cpu.register_a);
        cpu.update_flag(Flag::Negative, cpu.register_a)

    }, [
        (0x49, 2, 2, IMMEDIATE), 
        (0x45, 2, 3, ZERO_PAGE),
        (0x55, 2, 4, ZERO_PAGE_X),
        (0x4D, 3, 4, ABSOLUTE),
        (0x5D, 3, 4, ABSOLUTE_X), // +1 if page crossed
        (0x59, 3, 4, ABSOLUTE_Y), // +1 if page crossed
        (0x41, 2, 6, INDIRECT_X),
        (0x51, 2, 5, INDIRECT_Y), // +1 if page crossed
    ],

    // Logical Inclusive Or
    // A,Z,N = A|M
    ORA |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory, Flag};

        let addr = cpu.get_operand_addr(mode);
        let data = cpu.mem_read(addr);

        cpu.register_a |= data;

        cpu.update_flag(Flag::Zero, cpu.register_a);
        cpu.update_flag(Flag::Negative, cpu.register_a)

    }, [
        (0x09, 2, 2, IMMEDIATE), 
        (0x05, 2, 3, ZERO_PAGE),
        (0x15, 2, 4, ZERO_PAGE_X),
        (0x0D, 3, 4, ABSOLUTE),
        (0x1D, 3, 4, ABSOLUTE_X), // +1 if page crossed
        (0x19, 3, 4, ABSOLUTE_Y), // +1 if page crossed
        (0x01, 2, 6, INDIRECT_X),
        (0x22, 2, 5, INDIRECT_Y), // +1 if page crossed
    ],

    // Decrement memory
    DEC |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.decrement_memory(mode);
    }, [
        (0x25, 2, 5, ZERO_PAGE),
        (0x35, 2, 6, ZERO_PAGE_X),
        (0x2D, 3, 6, ABSOLUTE),
        (0x29, 3, 7, ABSOLUTE_X), 
    ],

    /* Shifts */
    // Arithmatic shift left
    // A,Z,C,N = M * 2 
    // or 
    // M,Z,C,N = M * 2
    ASL |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory, Flag};
        use crate::opcodes::AddressingMode;
        
        let mut data;

        match mode {
            AddressingMode::NONE_ADDRESSING => {
                data = cpu.register_a;

                // If a bit is left over set Carry flag
                match data >> 7 {
                    1 => cpu.status.insert(Flag::Carry),
                    _ => cpu.status.remove(Flag::Carry)
                }
        
                data <<= 1;
            }
            _ => {
                let addr = cpu.get_operand_addr(mode);
                data = cpu.mem_read(addr);
        
                // If a bit is left over set Carry flag
                match data >> 7 {
                    1 => cpu.status.insert(Flag::Carry),
                    _ => cpu.status.remove(Flag::Carry)
                }
        
                data <<= 1;
                cpu.mem_write(addr, data);
            }
        };

        cpu.update_flag(Flag::Zero, data);
        cpu.update_flag(Flag::Negative, data)

    }, [
        (0x2A, 1, 2, NONE_ADDRESSING), // Accumulator
        (0x26, 2, 5, ZERO_PAGE),
        (0x36, 2, 6, ZERO_PAGE_X),
        (0x2E, 3, 6, ABSOLUTE),
        (0x3E, 3, 7, ABSOLUTE_X),
    ],
    
    // Rotate Left
    ROL |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::opcodes::AddressingMode;

        if matches!(mode, AddressingMode::NONE_ADDRESSING) {
            cpu.rotate_left_a();
            return;
        }
        
        _ = cpu.rotate_left(mode);

    }, [
        (0x0A, 1, 2, NONE_ADDRESSING), // Accumulator
        (0x06, 2, 5, ZERO_PAGE),
        (0x16, 2, 6, ZERO_PAGE_X),
        (0x0E, 3, 6, ABSOLUTE),
        (0x1E, 3, 7, ABSOLUTE_X),
    ],

    ROR |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::opcodes::AddressingMode;

        if matches!(mode, AddressingMode::NONE_ADDRESSING) {
            cpu.rotate_right_a();
            return;
        }
        
        _ = cpu.rotate_right(mode);

    }, [
        (0x6A, 1, 2, NONE_ADDRESSING), // Accumulator
        (0x66, 2, 5, ZERO_PAGE),
        (0x76, 2, 6, ZERO_PAGE_X),
        (0x6E, 3, 6, ABSOLUTE),
        (0x7E, 3, 7, ABSOLUTE_X),
    ],

    // Decrement Memory
    DEM |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag, Memory};
        
        let addr = cpu.get_operand_addr(mode);
        let mut data = cpu.mem_read(addr);
        
        data = data.wrapping_sub(1);

        cpu.mem_write(addr, data);
        cpu.update_flag(Flag::Zero, data);
        cpu.update_flag(Flag::Negative, data);

    }, [
        (0xC6, 2, 5, ZERO_PAGE),
        (0xD6, 2, 6, ZERO_PAGE_X),
        (0xCE, 3, 6, ABSOLUTE),
        (0xDE, 3, 7, ABSOLUTE_X),
    ],

    // Decrement X Register
    // X,Z,N = X - 1
    DEX |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};
        
        cpu.register_x = cpu.register_x.wrapping_sub(1);
        cpu.update_flag(Flag::Zero, cpu.register_x);
        cpu.update_flag(Flag::Negative, cpu.register_x);
    }, [
        (0xCA, 1, 2, NONE_ADDRESSING),
    ],

    // Decrement Y Register
    // Y,Z,N = Y - 1
    DEY |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};
        
        cpu.register_y = cpu.register_y.wrapping_sub(1);
        cpu.update_flag(Flag::Zero, cpu.register_y);
        cpu.update_flag(Flag::Negative, cpu.register_y);
    }, [
        (0x88, 1, 2, NONE_ADDRESSING),
    ],

    CMP |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {

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

        cpu.compare(mode, cpu.register_x);
    }, [
        (0xE0, 2, 2, IMMEDIATE),
        (0xE4, 2, 3, ZERO_PAGE),
        (0xEC, 3, 4, ABSOLUTE),
    ],

    CPY |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {

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

    /* Branching */
    // +1 cycle length if branch succeeds +2 if a new page

    // Jump
    JMP |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use super::AddressingMode;

        cpu.jump(mode);
    }, [
        (0x4C, 3, 3, ABSOLUTE),
        (0x6C, 3, 5, NONE_ADDRESSING), // Indirect
    ],

    // Return from Subroutine
    RTS |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {

        cpu.counter = cpu.stack_pull_u16() + 1;
    }, [
        (0x60, 1, 6, NONE_ADDRESSING), // Implied
    ],

    RTI |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.status = Flag::from_bits_truncate(cpu.stack_pull());
        cpu.status.remove(Flag::BreakCommand);
        cpu.status.remove(Flag::BreakCommand2);

        cpu.counter = cpu.stack_pull_u16();
    }, [
        (0x40, 1, 6, NONE_ADDRESSING), // Implied
    ],

    // Jump to Subroutine
    JSR |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Memory};

        cpu.stack_push_u16(cpu.counter + 2 - 1);
        let target_addr = cpu.mem_read_u16(cpu.counter);
        cpu.counter = target_addr;

    }, [
        (0x20, 3, 6, ABSOLUTE),
    ],

    // Branch if carry clear
    BCC |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.branch(false == cpu.status.contains(Flag::Carry))
    }, [
        (0x90, 2, 2, NONE_ADDRESSING),
    ],

    // Branch if carry set
    BCS |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.branch(cpu.status.contains(Flag::Carry))
    }, [
        (0xB0, 2, 2, NONE_ADDRESSING),
    ],

    // Branch if equal
    BEQ |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.branch(cpu.status.contains(Flag::Zero))
    }, [
        (0xF0, 2, 2, NONE_ADDRESSING),
    ],

    // Branch if minus (Negative)
    BMI |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.branch(cpu.status.contains(Flag::Negative))
    }, [
        (0x30, 2, 2, NONE_ADDRESSING), 
    ],

    // Branch if not equal
    BNE |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.branch(false == cpu.status.contains(Flag::Zero))
    }, [
        (0xD0, 2, 2, NONE_ADDRESSING),
    ],

    // Branch if positive
    BPL |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.branch(false == cpu.status.contains(Flag::Negative))
    }, [
        (0x10, 2, 2, NONE_ADDRESSING),
    ],

    // Branch if overflow clear
    BVC |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.branch(false == cpu.status.contains(Flag::Overflow))
    }, [
        (0x50, 2, 2, NONE_ADDRESSING),
    ],

    // Branch if overflow set
    BVS |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.branch(cpu.status.contains(Flag::Overflow))
    }, [
        (0x70, 2, 2, NONE_ADDRESSING),
    ],

    // A & M, N = M7, V = M6
    BIT |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag, Memory};

        let addr = cpu.get_operand_addr(mode);
        let data = cpu.mem_read(addr);

        let flags = cpu.register_a & data;
        
        match flags {
            0 => cpu.status.insert(Flag::Zero),
            _ => cpu.status.remove(Flag::Zero)
        }

        // Bits 7 and 6 of data value is copied into Negative & Overflow flags
        cpu.status.set(Flag::Negative, data & 0b1000_0000 > 0);
        cpu.status.set(Flag::Overflow, data & 0b0100_0000 > 0);
    }, [
        (0x24, 2, 3, ZERO_PAGE),
        (0x2C, 3, 4, ABSOLUTE),
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

    // Clear Decimal Mode flag     
    // D = 0
    CLD |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.remove(crate::cpu::Flag::DecimalMode);
    }, [
        (0xD8, 1, 2, NONE_ADDRESSING),
    ],

    // Clear Interrupt Disable flag
    // I = 0
    CLI |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.remove(crate::cpu::Flag::InterruptDisable);
    }, [
        (0x58, 1, 2, NONE_ADDRESSING),
    ],

    // Clear Overflow Flag
    // V = 0 
    CLV |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.remove(crate::cpu::Flag::Overflow);
    }, [
        (0xB8, 1, 2, NONE_ADDRESSING),
    ],

    // Clear Carry flag
    CLC |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.remove(crate::cpu::Flag::Carry);  
    }, [
        (0x18, 1, 2, NONE_ADDRESSING),
    ],

    // https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
    TAX |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
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
    INX |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
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
    
    // Set Carry Flag
    // C = 1
    SEC |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.insert(crate::cpu::Flag::Carry);  
    }, [
        (0x38, 1, 2, NONE_ADDRESSING),
    ],

    // Set Interrupt Disable Flag
    // I = 1
    SEI |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.insert(crate::cpu::Flag::InterruptDisable);  
    }, [
        (0x78, 1, 2, NONE_ADDRESSING),
    ],
    
    // Set Decimal Flag (UNUSED)
    // D = 1
    SED |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.status.insert(crate::cpu::Flag::DecimalMode);  
    }, [
        (0xF8, 1, 2, NONE_ADDRESSING),
    ],

    // Transfer stack pointer to X
    // X = S
    TSX |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.register_x = cpu.stack_pointer;

        cpu.update_flag(Flag::Zero, cpu.register_x);
        cpu.update_flag(Flag::Negative, cpu.register_x);

    }, [
        (0xBA, 1, 2, NONE_ADDRESSING), // Implied
    ],

    // Transfer X to accumulator
    // A = X
    TXA |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.register_a = cpu.register_x;
        
        cpu.update_flag(Flag::Zero, cpu.register_a);
        cpu.update_flag(Flag::Negative, cpu.register_a);

    }, [
        (0x8A, 1, 2, NONE_ADDRESSING), // Implied
    ],

    // Transfer X to stack pointer
    // S = X
    TXS |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {

        cpu.stack_pointer = cpu.register_x;

    }, [
        (0x9A, 1, 2, NONE_ADDRESSING), // Implied
    ],

    // Transfer Y to accumulator
    // A = Y
    TYA |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::{Flag};

        cpu.register_a = cpu.register_y;
        
        cpu.update_flag(Flag::Zero, cpu.register_a);
        cpu.update_flag(Flag::Negative, cpu.register_a);

    }, [
        (0x98, 1, 2, NONE_ADDRESSING), // Implied
    ],

    /* Stack */
    PHA |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.stack_push(cpu.register_a);
    }, [
        (0x48, 1, 3, NONE_ADDRESSING),
    ],

    PHP |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        //http://wiki.nesdev.com/w/index.php/CPU_status_flag_behavior
        use crate::cpu::Flag;
        
        let mut flags = cpu.status.clone();
        flags.insert(Flag::BreakCommand);
        flags.insert(Flag::BreakCommand2);
        cpu.stack_push(flags.bits());
    }, [
        (0x08, 1, 3, NONE_ADDRESSING),
    ],

    PLA |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        cpu.register_a = cpu.stack_pull();
    }, [
        (0x68, 1, 4, NONE_ADDRESSING),
    ],

    PLP |cpu: &mut crate::cpu::CPU, mode: super::AddressingMode| {
        use crate::cpu::Flag;

        cpu.status = Flag::from_bits_truncate(cpu.stack_pull());
        cpu.status.remove(Flag::BreakCommand);
        cpu.status.remove(Flag::BreakCommand2);
    }, [
        (0x28, 1, 4, NONE_ADDRESSING),
    ]
];

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