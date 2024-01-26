use crate::oled_buffer::{OledBuffer, BUFFER_SIZE};

pub const VENDOR_ID: u16 = 0x1038;
pub const DEVICE_ID: u16 = 0x1610;
pub const OLED_WIDTH: usize = 128;
pub const OLED_HEIGHT: usize = 40;

pub struct Oled {
    keyboard_device: rusb::DeviceHandle<rusb::GlobalContext>
}

impl Oled {
    pub fn new() -> Result<Oled, ()> {
        return Self::new_from_ids(VENDOR_ID, DEVICE_ID);
    }

    pub fn new_from_ids(vendor_id: u16, device_id: u16) -> Result<Oled, ()> {
        let mut keyboard_device = None;

        let devices = match rusb::devices() {
            Ok(devices) => devices,
            Err(_) => return Err(())
        };

        for device in devices.iter() {
            let device_desc = match device.device_descriptor() {
                Ok(device_desc) => device_desc,
                Err(_) => continue
            };

            if device_desc.vendor_id() == vendor_id && device_desc.product_id() == device_id {
                keyboard_device = Some(device);
            }
        }

        let keyboard_device = match keyboard_device {
            Some(keyboard_device) => keyboard_device,
            None => return Err(())
        };
        let mut keyboard_device_handle = match keyboard_device.open() {
            Ok(handle) => handle,
            Err(_) => return Err(())
        };
        match keyboard_device_handle.claim_interface(1) {
            Ok(_) => { },
            Err(_) => return Err(())
        }

        Ok(Oled {
            keyboard_device: keyboard_device_handle
        })

    }

    pub fn dimensions() -> (usize, usize) {
        (OLED_WIDTH, OLED_HEIGHT)
    }

    pub fn blit(&self, buffer: &OledBuffer) {
        let mut oled_buffer = [0u8; BUFFER_SIZE / 8];
        buffer.to_buffer(&mut oled_buffer);
        let mut out_buffer = [0; (BUFFER_SIZE / 8) + 2];
        out_buffer[0] = 0x65;
        let buffer_len = out_buffer.len();
        out_buffer[1..buffer_len - 1].copy_from_slice(&oled_buffer);

        self.keyboard_device.write_control(0x21, 0x09, 0x0300, 1, &out_buffer, std::time::Duration::from_secs(1)).unwrap();
    }
}

impl Drop for Oled {
    fn drop(&mut self) {
        let _ = self.keyboard_device.release_interface(1);
    }
}