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
            buffer: vec![false; (height * scale) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        let index = (x + (y * self.width)) as usize;
        self.buffer[index] = true;
    }
}
