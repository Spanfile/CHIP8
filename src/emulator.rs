extern crate rand;

use self::rand::prelude::*;
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
    registers: [u8; 16],
    index: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    stack_pointer: u16,
    keypad: [u8; 16],
}

impl Emulator {
    pub fn new(window_scale: i32) -> Emulator {
        let screen_w = 64;
        let screen_h = 32;
        let mut emulator = Emulator {
            window: Window::new("chip8_window", "CHIP8", screen_w, screen_h, window_scale),
            memory: vec![0 as u8; 4096],
            program_counter: 0x200,
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

    pub fn cycle(&mut self) {
        let opcode: u16 = (self.memory[self.program_counter as usize] as u16) << 8
            | self.memory[(self.program_counter + 1) as usize] as u16;
        let x = (opcode >> 8) & 0x000F;
        let y = (opcode >> 4) & 0x000F;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        let x_usize = x as usize;
        let y_usize = y as usize;

        match opcode & 0xF000 {
            0x0000 => match nn {
                0xE0 => self.window.clear(),
                0xEE => {}
                _ => unknown_opcode(&opcode),
            },
            0x1000 => self.program_counter = nnn - 22,
            0x2000 => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.program_counter + 2;
                self.program_counter = nnn - 2;
            }
            0x3000 => {
                if self.registers[x_usize] == nn {
                    self.program_counter += 2;
                }
            }
            0x4000 => {
                if self.registers[x_usize] != nn {
                    self.program_counter += 2;
                }
            }
            0x5000 => {
                if self.registers[x_usize] == self.registers[y_usize] {
                    self.program_counter += 2;
                }
            }
            0x6000 => self.registers[x_usize] = nn,
            0x7000 => self.registers[x_usize] += nn,
            0x8000 => match n {
                0x00 => self.registers[x_usize] = self.registers[y_usize],
                0x01 => self.registers[x_usize] = self.registers[x_usize] | self.registers[y_usize],
                0x02 => self.registers[x_usize] = self.registers[x_usize] & self.registers[y_usize],
                0x03 => self.registers[x_usize] = self.registers[x_usize] ^ self.registers[y_usize],
                0x04 => {
                    self.registers[0xF] =
                        match self.registers[x_usize] as u16 + self.registers[y_usize] as u16 > 255
                        {
                            true => 1,
                            false => 0,
                        };
                    self.registers[x_usize] += self.registers[y_usize];
                }
                0x05 => {
                    self.registers[0xF] = match self.registers[x_usize] > self.registers[y_usize] {
                        true => 1,
                        false => 0,
                    };
                    self.registers[x_usize] -= self.registers[y_usize];
                }
                0x06 => {
                    self.registers[0xF] = self.registers[y_usize] & 0x01;
                    self.registers[x_usize] = self.registers[y_usize] >> 1;
                }
                0x07 => {
                    self.registers[0xF] = match self.registers[y_usize] > self.registers[x_usize] {
                        true => 1,
                        false => 0,
                    };
                    self.registers[x_usize] = self.registers[y_usize] - self.registers[x_usize];
                }
                0x0E => {
                    self.registers[0xF] = self.registers[y_usize] & 0x1;
                    self.registers[y_usize] = self.registers[y_usize] << 1;
                    self.registers[x_usize] = self.registers[y_usize];
                }
                _ => unknown_opcode(&opcode),
            },
            0x9000 => {
                if self.registers[x_usize] != self.registers[y_usize] {
                    self.program_counter += 2;
                }
            }
            0xA000 => self.index = nnn,
            0xB000 => self.program_counter = self.registers[0x0] as u16 + nnn,
            0xC000 => self.registers[x_usize] = random::<u8>() & nn,
            // 0xD000 => {} // TODO
            0xE000 => match nn {
                0x9E => if self.keypad[self.registers[x_usize] as usize] > 0 {
                    self.program_counter += 2;
                },
                0xA1 => if self.keypad[self.registers[x_usize] as usize] == 0 {
                    self.program_counter += 2;
                },
                _ => unknown_opcode(&opcode),
            },
            0xF000 => match nn {
                0x07 => self.registers[x_usize] = self.delay_timer,
                // 0x0A => {} // TODO
                0x15 => self.delay_timer = self.registers[x_usize],
                0x18 => self.sound_timer = self.registers[x_usize],
                0x1E => self.index += self.registers[x_usize] as u16,
                // 0x29 => {} // TODO
                // 0x33 => {} // TODO
                // 0x55 => {} // TODO
                // 0x65 => {} // TODO
                _ => unknown_opcode(&opcode),
            },
            _ => unknown_opcode(&opcode),
        }

        self.program_counter += 2;
    }

    fn draw(&mut self) {
        self.window.set_pixel(0, 0);
    }
}

fn unknown_opcode(opcode: &u16) {
    println!("unknown opcode 0x{:x}", &opcode)
}
