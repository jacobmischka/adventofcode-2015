use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let hp: i32 = lines
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

    let player = Player::new();
    let boss = Boss { hp, damage };

    let part1 = simulate(player.clone(), boss.clone(), false);
    println!("Part 1: {part1}");
    let part2 = simulate(player, boss, true);
    println!("Part 2: {part2}");
}

fn determine(mut player: Player, mut boss: Boss) {
    loop {
        if cfg!(feature = "debug") {
            eprintln!("\n-- Player turn --");
        }
        player.apply_effects(&mut boss);
        if boss.hp <= 0 {
            println!("Player won!");
            break;
        }

        player.choose_attack(&mut boss);
        if cfg!(feature = "debug") {
            dbg!(&player, &boss);
        }
        if boss.hp <= 0 {
            println!("Player won!");
            break;
        }

        if cfg!(feature = "debug") {
            eprintln!("\n-- Boss turn --");
        }

        player.apply_effects(&mut boss);
        if boss.hp <= 0 {
            println!("Player won!");
            break;
        }

        boss.attack(&mut player);

        if cfg!(feature = "debug") {
            dbg!(&player, &boss);
        }

        if player.hp <= 0 {
            println!("Boss won :(");
            break;
        }
    }

    println!("Part 1: {}", player.mana_used);
}

fn simulate(mut player: Player, mut boss: Boss, hard_mode: bool) -> u32 {
    let mut min_used = u32::MAX;

    if hard_mode {
        player.hp -= 1;

        if player.hp <= 0 {
            return min_used;
        }
    }

    player.apply_effects(&mut boss);

    if boss.hp <= 0 {
        return player.mana_used;
    }

    for attack in Attack::enumerate() {
        if !player.can_cast(attack) {
            continue;
        }

        let mut new_player = player.clone();
        let mut new_boss = boss.clone();

        new_player.use_attack(&mut new_boss, attack);

        if new_boss.hp <= 0 {
            if new_player.mana_used < min_used {
                min_used = new_player.mana_used;
            }
            continue;
        }

        new_player.apply_effects(&mut new_boss);
        if new_boss.hp <= 0 {
            if new_player.mana_used < min_used {
                min_used = new_player.mana_used;
            }
            continue;
        }

        new_boss.attack(&mut new_player);

        if new_player.hp <= 0 {
            continue;
        }

        let new_min = simulate(new_player, new_boss, hard_mode);
        if new_min < min_used {
            min_used = new_min;
        }
    }

    min_used
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Attack {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Attack {
    fn enumerate() -> [Attack; 5] {
        [
            Attack::MagicMissile,
            Attack::Drain,
            Attack::Shield,
            Attack::Poison,
            Attack::Recharge,
        ]
    }

    fn mana_cost(&self) -> i32 {
        match self {
            Attack::MagicMissile => 53,
            Attack::Drain => 73,
            Attack::Shield => 113,
            Attack::Poison => 173,
            Attack::Recharge => 229,
        }
    }

    fn damage(&self) -> i32 {
        match self {
            Attack::MagicMissile => 4,
            Attack::Drain => 2,
            Attack::Shield => 0,
            Attack::Poison => 3,
            Attack::Recharge => 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Boss {
    hp: i32,
    damage: i32,
}

impl Boss {
    fn attack(&mut self, player: &mut Player) {
        let net_damage = self.damage - player.armor;

        if cfg!(feature = "debug") {
            eprintln!("Boss attacks player for {net_damage} damage");
        }

        if net_damage > 0 {
            player.hp -= net_damage;
        }
    }
}

#[derive(Debug, Clone)]
struct Player {
    hp: i32,
    armor: i32,
    mana: i32,

    effects: HashMap<Attack, u32>,

    mana_used: u32,
}

impl Player {
    fn new() -> Self {
        Player {
            hp: 50,
            mana: 500,
            armor: 0,
            effects: HashMap::new(),
            mana_used: 0,
        }
    }

    fn can_cast(&self, attack: Attack) -> bool {
        self.mana >= attack.mana_cost() && !self.effects.contains_key(&attack)
    }

    fn choose_attack(&mut self, boss: &mut Boss) {
        let attack =
            if self.can_cast(Attack::MagicMissile) && boss.hp <= Attack::MagicMissile.damage() {
                Attack::MagicMissile
            } else if self.can_cast(Attack::Shield)
                && !(self.hp > boss.damage
                    && boss.hp < Attack::MagicMissile.damage() * 2 + Attack::Poison.damage() * 2)
            {
                Attack::Shield
            } else if self.can_cast(Attack::Recharge)
                && boss.hp > (Attack::MagicMissile.damage() + Attack::Poison.damage() * 2)
            //         * (self.mana / Attack::MagicMissile.mana_cost())
            // && self.effects.get(&Attack::Poison).copied().unwrap_or(0) as i32 / 2
            //     > (self.mana / Attack::MagicMissile.mana_cost())
            {
                Attack::Recharge
            } else if self.can_cast(Attack::Poison) && boss.hp > Attack::MagicMissile.damage() * 3 {
                Attack::Poison
            } else if self.can_cast(Attack::Drain) && self.hp + self.armor < boss.damage + 2 {
                Attack::Drain
            } else if self.can_cast(Attack::MagicMissile) {
                Attack::MagicMissile
            } else {
                panic!("Not enough mana :(")
            };

        self.use_attack(boss, attack);
    }

    fn use_attack(&mut self, boss: &mut Boss, attack: Attack) {
        let mana = attack.mana_cost();
        self.mana -= mana;
        self.mana_used += mana as u32;

        let damage = match attack {
            Attack::MagicMissile => attack.damage(),
            Attack::Drain => {
                self.hp += 2;
                attack.damage()
            }
            Attack::Shield => {
                self.armor += 7;
                self.effects.insert(attack, 6);
                0
            }
            Attack::Poison => {
                self.effects.insert(attack, 6);
                0
            }
            Attack::Recharge => {
                self.effects.insert(attack, 5);
                0
            }
        };

        if cfg!(feature = "debug") {
            eprintln!("Player uses {:?} for {} damage", attack, damage);
        }

        boss.hp -= damage;
    }

    fn apply_effects(&mut self, boss: &mut Boss) {
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
                    if cfg!(feature = "debug") {
                        eprintln!("Poison deals {} damage", effect.damage());
                    }
                    boss.hp -= effect.damage();
                }
                Attack::Recharge => {
                    if cfg!(feature = "debug") {
                        eprintln!("Recharge adds 101 mana");
                    }
                    self.mana += 101;
                }
            }
        }

        for attack in to_remove {
            if cfg!(feature = "debug") {
                eprintln!("{:?} expires", &attack);
            }
            self.effects.remove(&attack);
        }
    }
}
