#![feature(custom_attribute)]
mod emulator;
mod screen;
mod window;

use emulator::Emulator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = &args[1];

    let mut emulator = Emulator::new(20);
    emulator.load_rom(rom);

    loop {
        emulator.cycle();
        if !emulator.process_events() {
            break;
        }
    }
}
