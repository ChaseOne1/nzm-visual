use image::imageops::crop_imm;
use nzm_visual::constants::*;
use nzm_visual::recognizer;

fn main() {
    std::fs::create_dir_all(ATTACK_LEVELS_TEMPLATES_DIR)
        .expect("failed to create directory to save results");

    for i in ATTACK_LEVELS {
        if let Ok(source) = image::open(format!("{i}.png")) {
            let binary = recognizer::otsu_binarize(
                &crop_imm(&source, 160, 1446, 205 - 160, 1457 - 1446).to_image(),
            );
            if binary
                .save(format!(
                    "{ATTACK_LEVELS_TEMPLATES_DIR}/{i}.png",
                ))
                .is_ok()
            {
                println!("template {i}.png done")
            } else {
                eprintln!("failed to save template {i}.png");
            }
        } else {
            eprintln!("failed to open source {i}.png");
        }
    }
}
