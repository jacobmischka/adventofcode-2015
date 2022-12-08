use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut hp: i32 = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let damage: i32 = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut player = Player::new();
    loop {
        hp -= player.take_turn() as i32;
        if hp <= 0 {
            println!("Player won!");
            break;
        }

        player.hp -= damage;

        if player.hp <= 0 {
            println!("Boss won :(");
            break;
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Attack {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

struct Player {
    hp: i32,
    armor: u32,
    mana: u32,

    effects: HashMap<Attack, u32>,
}

impl Player {
    fn new() -> Self {
        Player {
            hp: 50,
            mana: 500,
            armor: 0,
            effects: HashMap::new(),
        }
    }

    fn take_turn(&mut self) -> u32 {
        let effect_damage = self.apply_effects();

        // FIXME
        let attack = Attack::MagicMissile;

        effect_damage + self.use_attack(attack)
    }

    fn use_attack(&mut self, attack: Attack) -> u32 {
        match attack {
            Attack::MagicMissile => {
                self.mana -= 53;
                4
            }
            Attack::Drain => {
                self.mana -= 73;
                self.hp += 2;
                2
            }
            Attack::Shield => {
                self.mana -= 113;
                self.effects.insert(attack, 6);
                0
            }
            Attack::Poison => {
                self.mana -= 173;
                self.effects.insert(attack, 6);
                0
            }
            Attack::Recharge => {
                self.mana -= 229;
                self.effects.insert(attack, 5);
                0
            }
        }
    }

    fn apply_effects(&mut self) -> u32 {
        let mut total_damage = 0;
        let mut to_remove: Vec<Attack> = Vec::new();

        for (effect, remaining) in &mut self.effects {
            let mut should_remove = false;
            *remaining -= 1;
            if *remaining == 0 {
                should_remove = true;
                to_remove.push(*effect);
            }

            match effect {
                Attack::MagicMissile => {}
                Attack::Drain => {}
                Attack::Shield => {
                    if should_remove {
                        self.armor -= 7;
                    }
                }
                Attack::Poison => {
                    total_damage += 3;
                }
                Attack::Recharge => {
                    self.mana += 101;
                }
            }
        }

        for attack in to_remove {
            self.effects.remove(&attack);
        }

        total_damage
    }
}
