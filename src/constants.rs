pub const MONEY_DIGIT_TEMPLATES_DIR: &str = "./money_digit_templates/";

pub const ATTACK_LEVELS_TEMPLATES_DIR: &str = "./attack_templates/";

pub const MONEY_DIGITS: std::ops::Range<u8> = 0..10;

pub const ATTACK_LEVELS: [u16; 29] = [
    25, 50, 75, 100, 125, 150, 175, 200, 225, 250, 275, 300, 325, 350, 375, 400, 425, 450, 475,
    500, 550, 600, 650, 700, 750, 800, 1000, 1200, 1500,
];

pub const ATTACK_WIDTHS_LEVEL_1: u16 = 50;
pub const ATTACK_WIDTHS_LEVEL_2: u16 = 450;
pub const ATTACK_WIDTHS_LEVEL_3: u16 = 1000;
pub const ATTACK_WIDTHS_LEVELS_NUM: [u16; 3] = [
    ATTACK_WIDTHS_LEVEL_1,
    ATTACK_WIDTHS_LEVEL_2,
    ATTACK_WIDTHS_LEVEL_3
];

pub const ATTACK_WIDTHS_LEVELS_1: [u16; 3] = [25, 50, 75];
pub const ATTACK_WIDTHS_LEVELS_2: [u16; 23] = [
    100, 125, 150, 175, 200, 225, 250, 275, 300, 325, 350, 375, 400, 425, 450, 475, 500, 550, 600,
    650, 700, 750, 800,
];
pub const ATTACK_WIDTHS_LEVELS_3: [u16; 3] = [1000, 1200, 1500];
