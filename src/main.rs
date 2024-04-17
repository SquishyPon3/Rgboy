mod cpu;
mod opcodes;
mod gamepad;

use cpu::CPU;

fn main() {

    // CPU.update()
    // BUS.update()
    // ROM.upate()
    // PPU.evaluate()
    // PPU.Render()
    // PPU.Scroll()
    // Gamepad.listen()
    // APU.update()

    

    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::ops::BitAnd;

    use crate::{cpu::Flag, opcodes::{INX_0xE8, BRK, CPY, LDA, TAX_0AA}};

    use super::*;
 
#[test]
fn test_0xa9_lda_immediate_load_data() {
    let mut cpu: CPU = CPU::new();
    let program = vec![
        LDA::IMMEDIATE::VALUE, 
        0x05, 
        BRK::NONE_ADDRESSING::VALUE
    ];

    cpu.load_and_run(program);

    assert!(
        cpu.status & Flag::from_bits_truncate(0b0000_0010) 
        == Flag::from_bits_retain(0b00));
    assert!(
        cpu.status & Flag::from_bits_truncate(0b1000_0000) 
        == Flag::from_bits_retain(0));
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu: CPU = CPU::new();
    let program = vec![
        LDA::IMMEDIATE::VALUE, 
        0x00, 
        BRK::NONE_ADDRESSING::VALUE
    ];

    cpu.load_and_run(program);
    
    assert!(
        cpu.status & Flag::from_bits_truncate(0b0000_0010) 
        == Flag::from_bits_retain(0b10));
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
        let mut cpu: CPU = CPU::new();
        let program = vec![
            TAX_0AA::NONE_ADDRESSING::VALUE, 
            BRK::NONE_ADDRESSING::VALUE
        ];
        
        cpu.load(program);
        cpu.reset_interrupt();
        cpu.register_a = 10;
        cpu.run();
    
        assert_eq!(cpu.register_x, 10)
}

#[test]
fn test_5_ops_working_together() {
        let mut cpu: CPU = CPU::new();
        
        let program = vec![
            LDA::IMMEDIATE::VALUE, 
            CPY::IMMEDIATE::VALUE,
            TAX_0AA::NONE_ADDRESSING::VALUE, 
            INX_0xE8::NONE_ADDRESSING::VALUE, 
            BRK::NONE_ADDRESSING::VALUE
        ];

        cpu.load_and_run(program);

        assert_eq!(cpu.register_x, 0xC1)    
}

#[test]
fn test_int_overflow() {
    let mut cpu: CPU = CPU::new();
    let program = vec![
        INX_0xE8::NONE_ADDRESSING::VALUE, 
        INX_0xE8::NONE_ADDRESSING::VALUE, 
        BRK::NONE_ADDRESSING::VALUE
    ];

    cpu.load(program);
    cpu.reset_interrupt();
    cpu.register_x = 0xff;
    cpu.run();

    assert_eq!(cpu.register_x, 1)
}
}
