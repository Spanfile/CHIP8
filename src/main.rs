mod window;

fn main() {
    let wndc = window::create_window("chip8_window", "CHIP8", 640, 320).unwrap();

    loop {
        if !window::handle_message(&wndc) {
            break;
        }
    }
}
