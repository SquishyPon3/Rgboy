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
    use crate::opcodes::{BRK_0x00, LDA};

    use super::*;
 
#[test]
fn test_0xa9_lda_immediate_load_data() {
    let mut cpu: CPU = CPU::new();
    let program = vec![
        0xA9, 
        0x05, 
        0x00];

    cpu.load_and_run(program);

    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu: CPU = CPU::new();
    let program = vec![
        LDA::IMMEDIATE_0xA9::VALUE, 
        BRK_0x00::VALUE, 
        BRK_0x00::VALUE];

    cpu.load_and_run(program);
    
    assert!(cpu.status & 0b0000_0010 == 0b10);
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
        let mut cpu: CPU = CPU::new();
        let program = vec![
            0xaa, 
            0x00];
        
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
            LDA::IMMEDIATE_0xA9::VALUE, 
            0xC0,
            0xAA, 
            0xE8, 
            0x00];

        cpu.load_and_run(program);

        assert_eq!(cpu.register_x, 0xC1)    
}

#[test]
fn test_int_overflow() {
    let mut cpu: CPU = CPU::new();
    let program = vec![
        0xe8, 
        0xe8, 
        0x00];

    cpu.load(program);
    cpu.reset_interrupt();
    cpu.register_x = 0xff;
    cpu.run();

    assert_eq!(cpu.register_x, 1)
}
}
