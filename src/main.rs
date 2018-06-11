#[macro_use]
extern crate lazy_static;

mod emulator;
mod screen;
mod window;

use emulator::Emulator;

fn main() {
    let emulator = Emulator::new(10);

    loop {
        if !emulator.process_events() {
            break;
        }
    }
}
