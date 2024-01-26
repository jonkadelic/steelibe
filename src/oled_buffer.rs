use crate::oled::OLED_WIDTH;
use crate::oled::OLED_HEIGHT;

pub const BUFFER_SIZE: usize = OLED_WIDTH * OLED_HEIGHT;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OledPixel {
    Off,
    On
}

impl OledPixel {
    pub fn pack(slice: &[OledPixel]) -> u8 {
        let mut out: u8 = 0;

        for val in slice.iter().enumerate() {
            let j = 7 - val.0;
            if *val.1 == OledPixel::On {
                out |= 1 << j;
            }
        }

        out
    }
}

#[derive(Clone, Copy)]
pub struct OledBuffer {
    buffer: [OledPixel; BUFFER_SIZE]
}

impl OledBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [OledPixel::Off; BUFFER_SIZE]
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<&OledPixel, ()> {
        if x >= OLED_WIDTH || y >= OLED_HEIGHT {
            return Err(());
        }

        let i = y * OLED_WIDTH + x;
        return Ok(&self.buffer[i]);
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Result<&mut OledPixel, ()> {
        if x >= OLED_WIDTH || y >= OLED_HEIGHT {
            return Err(());
        }

        let i = y * OLED_WIDTH + x;
        return Ok(&mut self.buffer[i]);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, state: OledPixel) {
        if x >= OLED_WIDTH || y >= OLED_HEIGHT {
            return;
        }

        let i = y * OLED_WIDTH + x;
        self.buffer[i] = state;
    }

    pub fn clear(&mut self) {
        for i in 0..BUFFER_SIZE {
            self.buffer[i] = OledPixel::Off;
        }
    }

    pub fn to_buffer(&self, buffer: &mut [u8; BUFFER_SIZE / 8]) {
        for y in 0..OLED_HEIGHT {
            for x in (0..OLED_WIDTH).step_by(8) {
                let i = y * OLED_WIDTH + x;
                let slice = &self.buffer[i..(i + 8)];
                buffer[i / 8] = OledPixel::pack(slice);
            }
        }
    } 
}