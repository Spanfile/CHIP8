use std::fs::File;
use std::io::prelude::*;
use window::Window;

#[rustfmt_skip]
static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Default)]
pub struct Emulator {
    window: Window,
    memory: Vec<u8>,
    registers: Vec<u8>,
    index: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<u16>,
    stack_pointer: u16,
}

impl Emulator {
    pub fn new(window_scale: i32) -> Emulator {
        let screen_w = 64;
        let screen_h = 32;
        let mut emulator = Emulator {
            window: Window::new("chip8_window", "CHIP8", screen_w, screen_h, window_scale),
            memory: vec![0 as u8; 4096],
            registers: vec![0 as u8, 16],
            stack: vec![0 as u16, 16],
            ..Default::default()
        };
        emulator.load_fontset();
        emulator
    }

    pub fn process_events(&self) -> bool {
        self.window.dispatch_messages()
    }

    pub fn load_rom(&mut self, filename: &str) {
        println!("load {}", filename);
        let mut f = File::open(filename).expect("ROM file not found");
        f.read(&mut self.memory[0x200..])
            .expect("couldn't read ROM into memory");
    }

    fn load_fontset(&mut self) {
        self.memory[..80].clone_from_slice(&FONTSET);
    }

    pub fn cycle(&mut self) {}

    fn draw(&mut self) {
        self.window.set_pixel(0, 0);
    }
}
