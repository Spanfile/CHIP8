extern crate libc;
use std::mem;

#[derive(Default)]
pub struct Screen {
    pub width: u8,
    pub height: u8,
    pub scale: i32,
    pub buffer: Vec<bool>,
}

impl Screen {
    pub fn new(width: u8, height: u8, scale: i32) -> Screen {
        Screen {
            width,
            height,
            scale,
            buffer: vec![false; (width as i32 * height as i32) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, on: bool) {
        let index = (x + (y * self.width)) as usize;
        self.buffer[index] = on;
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        let index = (x + (y * self.width)) as usize;
        self.buffer[index]
    }

    pub fn clear(&mut self) {
        unsafe {
            libc::memset(
                self.buffer.as_mut_ptr() as _,
                0,
                self.buffer.len() * mem::size_of::<bool>(),
            );
        }
    }
}
