mod window;
use window::Window;

fn main() {
    let window = Window::new("chip8_window", "CHIP8", 1280, 640);

    loop {
        if !window.dispatch_messages() {
            break;
        }
    }
}
