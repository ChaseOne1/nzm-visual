use dxgi_capture_rs::DXGIManager;
use image::{ImageBuffer, Rgba};

pub struct Capturer {
    manager: DXGIManager,
}

impl Capturer {
    pub fn new() -> Self {
        Self {
            manager: DXGIManager::new(5000).unwrap(),
        }
    }

    pub fn capture(&mut self) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        match self.manager.capture_frame_components() {
            Ok((pixels, (frame_width, frame_height))) => {
                let mut screenshot = ImageBuffer::<Rgba<u8>, _>::from_raw(
                    frame_width as u32,
                    frame_height as u32,
                    pixels,
                )?;
                for pix in screenshot.pixels_mut() {
                    pix.0[..3].reverse();
                }
                Some(screenshot)
            }
            Err(e) => {
                eprintln!("捕获失败: {:?}", e);
                None
            }
        }
    }
}
