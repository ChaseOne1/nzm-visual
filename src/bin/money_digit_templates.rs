use nzm_visual::recognizer;

//NOTE: just binarize, the digit images needs to be manully cropped

fn main() {
    let dir = "./money_digit_templates";
    std::fs::create_dir_all(dir).expect("failed to create directory to save results");

    for i in 0..10 {
        if let Ok(source) = image::open(format!("{i}.png")) {
            let binary = recognizer::otsu_binarize(&source);
            if binary.save(format!("{dir}/{i}.png", )).is_err() {
                eprintln!("failed to save template {i}.png");
            }
        } else {
            eprintln!("failed to open {i}.png")
        }
    }
}
