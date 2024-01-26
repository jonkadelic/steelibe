mod oled;
mod oled_buffer;

pub use oled::Oled;
pub use oled_buffer::OledBuffer;
pub use oled_buffer::OledPixel;

#[cfg(test)]
mod tests {
    use crate::{oled::{OLED_HEIGHT, OLED_WIDTH}, oled_buffer::{OledBuffer, OledPixel, BUFFER_SIZE}, Oled};

    #[test]
    fn test_oled_new() {
        let oled = Oled::new();
        assert!(oled.is_ok())
    }

    #[test]
    fn test_oled_new_vid_did() {
        let oled = Oled::new_from_ids(crate::oled::VENDOR_ID, crate::oled::DEVICE_ID);
        assert!(oled.is_ok());
        let oled = Oled::new_from_ids(0x0000, 0x0000);
        assert!(oled.is_err());
    }

    #[test]
    fn test_oled_dimensions() {
        let (width, height) = Oled::dimensions();
        assert_eq!(width, crate::oled::OLED_WIDTH);
        assert_eq!(height, crate::oled::OLED_HEIGHT);
    }

    #[test]
    fn test_oled_buffer_get_set_pixel() {
        let mut oled_buffer = OledBuffer::new();
        let px = oled_buffer.get_pixel(0, 0);
        assert!(px.is_ok());
        assert_eq!(*px.unwrap(), OledPixel::Off);
        
        let px = oled_buffer.get_pixel(usize::MAX, usize::MAX);
        assert!(px.is_err());

        oled_buffer.set_pixel(0, 0, OledPixel::On);
        let px = oled_buffer.get_pixel(0, 0).unwrap();
        assert_eq!(*px, OledPixel::On);
    }

    #[test]
    fn test_oled_buffer_to_buffer() {
        let mut oled_buffer = OledBuffer::new();
        oled_buffer.set_pixel(0, 0, OledPixel::On);

        let buffer = &mut [0u8; BUFFER_SIZE / 8];
        oled_buffer.to_buffer(buffer);
        
        assert_eq!(buffer[0], 0x80);
        for i in 1..buffer.len() {
            assert_eq!(buffer[i], 0x00);
        }
    }

    #[test]
    fn test_oled_write_test_pattern() {
        let oled = Oled::new();
        assert!(oled.is_ok());
        let oled = oled.unwrap();
        let mut oled_buffer = OledBuffer::new();

        for y in 0..OLED_HEIGHT {
            for x in 0..OLED_WIDTH {
                let mod_x = x % 2;
                let mod_y = y % 2;
                let mut state = OledPixel::Off;
                if mod_y == 0 {
                    if (mod_y == 0 && mod_x == 1) || (mod_y == 1 && mod_x == 0) {
                        state = OledPixel::On;
                    }
                }
                oled_buffer.set_pixel(x, y, state);
            }
        }

        oled.blit(&oled_buffer);
    }
}