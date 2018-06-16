use window::Window;

#[derive(Default)]
pub struct Emulator {
    _window: Window,
}

impl Emulator {
    pub fn new(window_scale: i32) -> Emulator {
        let screen_w = 64;
        let screen_h = 32;
        Emulator {
            _window: Window::new("chip8_window", "CHIP8", screen_w, screen_h, window_scale),
            ..Default::default()
        }
    }

    pub fn process_events(&self) -> bool {
        self._window.dispatch_messages()
    }

    pub fn draw(&mut self) {
        self._window.set_pixel(0, 0);
    }
}
