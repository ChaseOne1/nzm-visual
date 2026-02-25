const LEVEL_DATA: [(u16, u32); 30] = [
    (0, 400),
    (25, 500),
    (50, 600),
    (75, 800),
    (100, 1000),
    (125, 1200),
    (150, 1500),
    (175, 1800),
    (200, 2100),
    (225, 2500),
    (250, 2900),
    (275, 3300),
    (300, 3800),
    (325, 4300),
    (350, 4800),
    (375, 5400),
    (400, 6000),
    (425, 6600),
    (450, 7400),
    (475, 8200),
    (500, 9200),
    (550, 10200),
    (600, 11400),
    (650, 12600),
    (700, 15000),
    (750, 19800),
    (800, 39800),
    (1000, 59800),
    (1200, 79800),
    (1500, 0),
];

pub struct Processor {
    last_money: u32,
    last_attack: u16,
}

impl Processor {
    fn process_money(&self, (money, _score): (u32, u32)) -> Option<u32> {
        Some(money)
    }

    const ATTACK_PASSING_SCORE: u32 = 100;
    fn process_attack(&self, (attack, score): (u16, u32)) -> Option<u16> {
        if score <= Self::ATTACK_PASSING_SCORE{
            Some(attack)
        } else {
            Some(0)
        }
    }

    pub fn new() -> Self {
        Self {
            last_money: 0,
            last_attack: 0,
        }
    }

    pub fn process(&self, money: (u32, u32), attack: (u16, u32)) -> (u32, u16) {
        let money = self.process_money(money).unwrap();
        let attack = self.process_attack(attack).unwrap();

        let mut remaining_money = money;
        let mut final_attack = attack;

        if let Some(level) = LEVEL_DATA.iter().position(|&(a, _)| a == attack) {
            for i in (level + 1)..30 {
                let cost = LEVEL_DATA[i - 1].1;
                if cost == 0 || remaining_money < cost {
                    break;
                }
                remaining_money -= cost;
                final_attack = LEVEL_DATA[i].0;
            }
        }

        (remaining_money, final_attack)
    }
}
