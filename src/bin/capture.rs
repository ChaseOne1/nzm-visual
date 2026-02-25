use dxgi_capture_rs::DXGIManager;
use image::{RgbImage};

fn screenshot() {
    println!("初始化DXGI...");
    let mut manager = DXGIManager::new(5000).unwrap();

    let (width, height) = manager.geometry();
    println!("屏幕分辨率: {}x{}", width, height);

    std::thread::sleep(std::time::Duration::from_secs(1));

    match manager.capture_frame() {
        Ok((pixels, (frame_width, frame_height))) => {
            println!("捕获尺寸 {}x{}", frame_width, frame_height);

            let mut raw = Vec::with_capacity(frame_width * frame_height * 3);
            for pixel in pixels {
                raw.push(pixel.r);
                raw.push(pixel.g);
                raw.push(pixel.b);
            }

            let img = RgbImage::from_raw(frame_width as u32, frame_height as u32, raw).unwrap();
            img.save("capture.png").unwrap();
            println!("截图保存为 capture.png");
        }
        Err(e) => {
            println!("捕获失败: {:?}", e);
        }
    }
}

fn main() {
    screenshot();
}
