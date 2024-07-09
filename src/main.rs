mod cpu;
mod opcodes;
mod gamepad;

use core::time;

use cpu::{Memory, CPU};
use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, pixels::{Color, PixelFormat, PixelFormatEnum}, EventPump};

fn main() {

    // CPU.update()
    // BUS.update()
    // ROM.upate()
    // PPU.evaluate()
    // PPU.Render()
    // PPU.Scroll()
    // Gamepad.listen()
    // APU.update()
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake!", (32 * 10) as u32, (32 * 10) as u32 )
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32).unwrap();

    let mut cpu = CPU::new();
    cpu.load_snake();
    cpu.reset_interrupt();

    let mut screen_state = [0 as u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();

    cpu.run_snake_with_callback(move |cpu| {
        
        handle_input(cpu, &mut event_pump);
        cpu.mem_write(0xFE, rng.gen_range(1..16));

        if read_screen_state(cpu, &mut screen_state) {
            texture.update(None, &screen_state, 32 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }
        
        std::thread::sleep(time::Duration::new(0, 70_000))
    });
}

fn handle_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0)
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                cpu.mem_write(0xFF, 0x77)
            }
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                cpu.mem_write(0xFF, 0x73)
            }
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                cpu.mem_write(0xFF, 0x61)
            }
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                cpu.mem_write(0xFF, 0x64)
            }
            _ => {/* Do nothing! */}
        }
    }
}

fn to_color(byte: u8) -> Color {
    return match byte {
        0 => sdl2::pixels::Color::BLACK,
        1 => sdl2::pixels::Color::WHITE,
        2 | 9 => sdl2::pixels::Color::GREY,
        3 | 10 => sdl2::pixels::Color::RED,
        4 | 11 => sdl2::pixels::Color::GREEN,
        5 | 12 => sdl2::pixels::Color::BLUE,
        6 | 13 => sdl2::pixels::Color::MAGENTA,
        7 | 14 => sdl2::pixels::Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    
    for i in 0x0200..0x600 {
        let color_idx = cpu.mem_read(i as u16);
        let (b1, b2, b3) = to_color(color_idx).rgb();

        if frame[frame_idx] != b1 
        || frame[frame_idx + 1] != b2 
        || frame[frame_idx + 2] != b3 {

            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;

        }

        frame_idx += 3;
    }

    return update;
}

#[cfg(test)]
mod test {
    use std::ops::BitAnd;

    use opcodes::{ADC, CLC, SBC, SEC};

    use crate::{cpu::Flag, opcodes::{INX, BRK, CPY, LDA, TAX}};

    use super::*;

// #[test]
// fn test_lda_zero_page_load_data() {
//     let mut cpu: CPU = CPU::new();
//     let program = vec![
//         CLC::NONE_ADDRESSING::VALUE, 
//         LDA::IMMEDIATE::VALUE,
//         0x02,
//         ADC::IMMEDIATE::VALUE,
//         0x03,
//         SEC::NONE_ADDRESSING::VALUE,
//         LDA::IMMEDIATE::VALUE,
//         0x15,
//         SBC::IMMEDIATE::VALUE,
//         0x08,
//         BRK::NONE_ADDRESSING::VALUE
//     ];

//     cpu.load_and_run(program);
// }

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
            TAX::NONE_ADDRESSING::VALUE, 
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
            TAX::NONE_ADDRESSING::VALUE, 
            INX::NONE_ADDRESSING::VALUE, 
            BRK::NONE_ADDRESSING::VALUE
        ];

        cpu.load_and_run(program);

        assert_eq!(cpu.register_x, 0xC1)    
}

#[test]
fn test_int_overflow() {
    let mut cpu: CPU = CPU::new();
    let program = vec![
        INX::NONE_ADDRESSING::VALUE, 
        INX::NONE_ADDRESSING::VALUE, 
        BRK::NONE_ADDRESSING::VALUE
    ];

    cpu.load(program);
    cpu.reset_interrupt();
    cpu.register_x = 0xff;
    cpu.run();

    assert_eq!(cpu.register_x, 1)
}

#[test]
fn test_match_bit_and() {

    let a = 32 & 1;
    let b = 16 & 3;
    let c = 183 & 32;

    let a_match = match a {
        1 => true,
        _ => false
    };

    let mut a_if;
    if a == 1 {
        a_if = true;
    } else {
        a_if = false;
    }

    let b_match = match b {
        1 => true,
        _ => false
    };

    let mut b_if;
    if b == 1 {
        b_if = true;
    } else {
        b_if = false;
    }

    let c_match = match c {
        1 => true,
        _ => false
    };

    let mut c_if;
    if c == 1 {
        c_if = true;
    } else {
        c_if = false;
    }

    assert_eq!(a_match, a_if);
    assert_eq!(b_match, b_if);
    assert_eq!(c_match, c_if);
}
}