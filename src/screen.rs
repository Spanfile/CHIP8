#[derive(Default)]
pub struct Screen {
    pub width: i32,
    pub height: i32,
    pub scale: i32,
    pub buffer: Vec<bool>,
}

impl Screen {
    pub fn new(width: i32, height: i32, scale: i32) -> Screen {
        Screen {
            width,
            height,
            scale,
            buffer: vec![false; (width * height) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, on: bool) {
        let index = (x + (y * self.width)) as usize;
        self.buffer[index] = on;
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> bool {
        let index = (x + (y * self.width)) as usize;
        self.buffer[index]
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = false;
        }
    }
}
