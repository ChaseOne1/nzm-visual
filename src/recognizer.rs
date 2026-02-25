use std::{collections::HashMap, sync::OnceLock};

use image::{GenericImageView, GrayImage, Pixel, imageops};

use crate::constants::*;

const COMMA_WIDTH: u8 = 3;

fn otsu_threshold(gray: &GrayImage) -> u8 {
    let mut hist = [0u32; 256];
    let gray_data = gray.as_raw();
    for &pixel in gray_data {
        hist[pixel as usize] += 1;
    }

    let total_pixels = gray_data.len() as u32;

    let mut cum_pixels = [0u32; 256];
    let mut cum_sum = [0u64; 256];
    cum_pixels[0] = hist[0];
    cum_sum[0] = (0 * hist[0]) as u64;

    for i in 1..256 {
        cum_pixels[i] = cum_pixels[i - 1] + hist[i];
        cum_sum[i] = cum_sum[i - 1] + (i as u64) * (hist[i] as u64);
    }

    let total_sum = cum_sum[255];

    let mut best_threshold = 0;
    let mut max_variance = 0.0;

    for t in 0..256 {
        let pixels_bg = cum_pixels[t];
        let pixels_fg = total_pixels - pixels_bg;

        if pixels_bg == 0 || pixels_fg == 0 {
            continue;
        }

        let sum_bg = cum_sum[t];
        let sum_fg = total_sum - sum_bg;

        let mean_bg = sum_bg as f64 / pixels_bg as f64;
        let mean_fg = sum_fg as f64 / pixels_fg as f64;

        let variance = (pixels_bg as f64) * (pixels_fg as f64) * (mean_bg - mean_fg).powi(2);

        if variance > max_variance {
            max_variance = variance;
            best_threshold = t;
        }
    }

    best_threshold as u8
}

pub fn otsu_binarize<T>(img: &T) -> GrayImage
where
    T: GenericImageView,
    T::Pixel: Pixel<Subpixel = u8>,
{
    let gray: GrayImage = imageops::grayscale(img);
    let threshold = otsu_threshold(&gray);

    let gray_data = gray.as_raw();
    let binary_data: Vec<u8> = gray_data
        .iter()
        .map(|&pixel| if pixel > threshold { 255 } else { 0 })
        .collect();

    GrayImage::from_vec(img.width(), img.height(), binary_data)
        .expect("Failed to create binary image")
}

// NOTE: left close right open
pub fn vertical_digit_divide(binary: &GrayImage) -> Vec<(u8, u8)> {
    let mut intervals: Vec<(u8, u8)> = Vec::new();
    let (mut in_character, mut char_begin) = (false, 0u8);

    let width = binary.width() as usize;
    let height = binary.height() as usize;
    let binary_data = binary.as_raw();

    for x in 0..width {
        let mut has_white = false;
        for y in 0..height {
            let idx = y * width + x;
            if binary_data[idx] != 0 {
                has_white = true;
                break;
            }
        }

        match (in_character, has_white) {
            (false, true) => {
                in_character = true;
                char_begin = x as u8;
            }
            (true, false) => {
                in_character = false;
                if (x as u8 - char_begin) > COMMA_WIDTH {
                    intervals.push((char_begin, x as u8));
                }
            }
            _ => {}
        }
    }

    if in_character {
        let char_width = (width - 1) as u8;
        if char_width > COMMA_WIDTH {
            intervals.push((char_begin, char_width));
        }
    }

    intervals
}

pub fn vertical_number_divide(binary: &GrayImage) -> (u8, u8) {
    let mut intervals: (u8, u8) = (0, 0);

    let width = binary.width() as usize;
    let height = binary.height() as usize;
    let binary_data = binary.as_raw();

    'begin: for x in 0..width {
        for y in 0..height {
            let idx = y * width + x;
            if binary_data[idx] != 0 {
                intervals.0 = x as u8;
                break 'begin;
            }
        }
    }

    'end: for x in (0..width).rev() {
        for y in 0..height {
            let idx = y * width + x;
            if binary_data[idx] != 0 {
                intervals.1 = (x + 1) as u8;
                break 'end;
            }
        }
    }

    intervals
}

pub fn bidirectional_distance_transform(binary: &GrayImage) -> (GrayImage, GrayImage) {
    let (width, height) = binary.dimensions();
    let width_usize = width as usize;
    let height_usize = height as usize;
    let total_pixels = width_usize * height_usize;

    let binary_data = binary.as_raw();
    let mut fg_dist_data = vec![u8::MAX; total_pixels];
    let mut bg_dist_data = vec![u8::MAX; total_pixels];

    for y in 0..height_usize {
        let row_offset = y * width_usize;
        for x in 0..width_usize {
            let idx = row_offset + x;
            let is_foreground = binary_data[idx] == 255;

            if is_foreground {
                fg_dist_data[idx] = 0;
                bg_dist_data[idx] = u8::MAX;
            } else {
                fg_dist_data[idx] = u8::MAX;
                bg_dist_data[idx] = 0;
            }

            if y > 0 {
                let up_idx = idx - width_usize;
                fg_dist_data[idx] = fg_dist_data[idx].min(fg_dist_data[up_idx].saturating_add(1));
                bg_dist_data[idx] = bg_dist_data[idx].min(bg_dist_data[up_idx].saturating_add(1));
            }

            if x > 0 {
                let left_idx = idx - 1;
                fg_dist_data[idx] = fg_dist_data[idx].min(fg_dist_data[left_idx].saturating_add(1));
                bg_dist_data[idx] = bg_dist_data[idx].min(bg_dist_data[left_idx].saturating_add(1));
            }
        }
    }

    for y in (0..height_usize).rev() {
        let row_offset = y * width_usize;
        for x in (0..width_usize).rev() {
            let idx = row_offset + x;

            if y < height_usize - 1 {
                let down_idx = idx + width_usize;
                fg_dist_data[idx] = fg_dist_data[idx].min(fg_dist_data[down_idx].saturating_add(1));
                bg_dist_data[idx] = bg_dist_data[idx].min(bg_dist_data[down_idx].saturating_add(1));
            }

            if x < width_usize - 1 {
                let right_idx = idx + 1;
                fg_dist_data[idx] =
                    fg_dist_data[idx].min(fg_dist_data[right_idx].saturating_add(1));
                bg_dist_data[idx] =
                    bg_dist_data[idx].min(bg_dist_data[right_idx].saturating_add(1));
            }
        }
    }

    (
        GrayImage::from_vec(width, height, fg_dist_data)
            .expect("Failed to create foreground distance image"),
        GrayImage::from_vec(width, height, bg_dist_data)
            .expect("Failed to create background distance image"),
    )
}

pub fn template_match(src_binary: &GrayImage, tmpl_fg: &GrayImage, tmpl_bg: &GrayImage) -> u32 {
    let src_binary_data = src_binary.as_raw();
    let tmpl_fg_data = tmpl_fg.as_raw();
    let tmpl_bg_data = tmpl_bg.as_raw();

    let mut score = 0u32;

    for i in 0..src_binary_data.len().min(tmpl_fg_data.len()) {
        score += if src_binary_data[i] == 255 {
            tmpl_fg_data[i] as u32
        } else {
            tmpl_bg_data[i] as u32
        };
    }

    if cfg!(feature = "score_log") {
        println!("{score}")
    }

    score
}

// fg_dist bg_dist
pub static MONEY_DIGIT_DISTANCES: OnceLock<HashMap<u8, (GrayImage, GrayImage)>> = OnceLock::new();
pub static ATTACK_DISTANCES: OnceLock<HashMap<u16, (GrayImage, GrayImage)>> = OnceLock::new();
pub static ATTACK_NUMBER_WIDTHS: OnceLock<HashMap<u16, u8>> = OnceLock::new();

pub fn initialize() {
    let mut distances = HashMap::new();
    for i in MONEY_DIGITS {
        if let Ok(source) = image::open(format!("{MONEY_DIGIT_TEMPLATES_DIR}/{i}.png")) {
            let binary = source.to_luma8();
            distances.insert(i, bidirectional_distance_transform(&binary));
        } else {
            panic!("failed to load template {i}");
        }
    }
    MONEY_DIGIT_DISTANCES.set(distances).unwrap();

    let mut distances = HashMap::new();
    let mut widths = HashMap::new();
    for i in ATTACK_LEVELS {
        if let Ok(source) = image::open(format!("{ATTACK_LEVELS_TEMPLATES_DIR}/{i}.png")) {
            let binary = source.to_luma8();
            distances.insert(i, bidirectional_distance_transform(&binary));
            if ATTACK_WIDTHS_LEVELS_NUM.contains(&i) {
                let (begin, end) = vertical_number_divide(&binary);
                widths.insert(i, end - begin);
            }
        } else {
            panic!("failed to load template {}", i);
        }
    }
    ATTACK_DISTANCES.set(distances).unwrap();
    ATTACK_NUMBER_WIDTHS.set(widths).unwrap();
}

//TODO: recognize failed will be 1, need fix
pub fn recognize_money<T>(source: &T) -> (u32, u32)
where
    T: GenericImageView<Pixel: Pixel<Subpixel = u8>>,
{
    let binary = otsu_binarize(source);
    let (_, height) = binary.dimensions();
    let (mut money, mut score) = (0u32, 0u32);

    for &(begin, end) in &vertical_digit_divide(&binary) {
        let cropped = {
            let single = imageops::crop_imm(&binary, begin as u32, 0, (end - begin) as u32, height)
                .to_image();
            if single.width()
                == MONEY_DIGIT_DISTANCES
                    .get()
                    .unwrap()
                    .get(&0)
                    .unwrap()
                    .0
                    .width()
            {
                single
            } else {
                imageops::resize(
                    &single,
                    MONEY_DIGIT_DISTANCES
                        .get()
                        .unwrap()
                        .get(&0)
                        .unwrap()
                        .0
                        .width(),
                    MONEY_DIGIT_DISTANCES
                        .get()
                        .unwrap()
                        .get(&0)
                        .unwrap()
                        .0
                        .height(),
                    imageops::FilterType::Nearest,
                )
            }
        };

        let result = MONEY_DIGITS
            .map(|i| {
                if cfg!(feature = "score_log") {
                    print!("digit {i} score: ");
                }
                let (tmpl_fg, tmpl_bg) = MONEY_DIGIT_DISTANCES.get().unwrap().get(&i).unwrap();
                (i, template_match(&cropped, tmpl_fg, tmpl_bg))
            })
            .min_by_key(|&(_, score)| score)
            .unwrap();

        if cfg!(feature = "score_log") {
            println!("--------RESULT: {}---------", result.0);
        }
        money = money * 10 + result.0 as u32;
        score += result.1;
    }

    (money, score)
}

pub fn recognize_attack<T>(source: &T) -> (u16, u32)
where
    T: GenericImageView<Pixel: Pixel<Subpixel = u8>>,
{
    let binary = otsu_binarize(source);
    let (begin, end) = vertical_number_divide(&binary);
    let width = end - begin;

    let similar = ATTACK_WIDTHS_LEVELS_NUM
        .iter()
        .min_by_key(|&x| {
            let w = ATTACK_NUMBER_WIDTHS.get().unwrap().get(x).unwrap();
            std::cmp::max(*w, width) - std::cmp::min(*w, width)
        })
        .unwrap();

    let candidate: Vec<u16> = match *similar {
        ATTACK_WIDTHS_LEVEL_1 => ATTACK_WIDTHS_LEVELS_1.to_vec(),
        ATTACK_WIDTHS_LEVEL_2 => ATTACK_WIDTHS_LEVELS_2.to_vec(),
        ATTACK_WIDTHS_LEVEL_3 => ATTACK_WIDTHS_LEVELS_3.to_vec(),
        _ => Vec::new(),
    };

    candidate
        .iter()
        .map(|&atk| {
            if cfg!(feature = "score_log") {
                print!("attack {atk} score: ");
            }

            let (tmpl_fg, tmpl_bg) = ATTACK_DISTANCES.get().unwrap().get(&atk).unwrap();
            (atk, template_match(&binary, tmpl_fg, tmpl_bg))
        })
        .min_by_key(|&(_, score)| score)
        .unwrap()
}
