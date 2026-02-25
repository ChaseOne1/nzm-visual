use image::imageops;
use std::time::Instant;

use nzm_visual::capturer;
use nzm_visual::overlayer;
use nzm_visual::processor;
use nzm_visual::recognizer;

fn main() {
    recognizer::initialize();
    overlayer::Overlayer::initialize();

    let mut capturer = capturer::Capturer::new();
    let processor = processor::Processor::new();
    let wnd = overlayer::Overlayer::new(100, 800, 300, 150);
    std::thread::sleep(std::time::Duration::from_secs(1)); // waiting for capture to initialize

    loop {
        if let Some(screenshot) = capturer.capture() {
            let start = Instant::now();

            let money = recognizer::recognize_money(
                &imageops::crop_imm(&screenshot, 67, 1387, 144 - 66, 1402 - 1387).to_image(),
            );
            let attack = recognizer::recognize_attack(
                &imageops::crop_imm(&screenshot, 160, 1446, 205 - 160, 1457 - 1446).to_image(),
            );

            let duration = start.elapsed();

            let future_data = processor.process(money, attack);

            wnd.tick(
                money.0,
                money.1,
                attack.0,
                attack.1,
                future_data.0,
                future_data.1,
                duration,
            );
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
