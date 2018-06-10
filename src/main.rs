mod window;
use window::Window;

fn main() {
    let window = Window::new("chip8_window", "CHIP8", 640, 320);

    loop {
        if !window.dispatch_messages() {
            break;
        }
    }
}
