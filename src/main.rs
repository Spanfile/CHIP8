mod emulator;
mod screen;
mod window;

use emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new(20);
    emulator.draw();

    loop {
        if !emulator.process_events() {
            break;
        }
    }
}
