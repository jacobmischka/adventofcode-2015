use std::{
    collections::BTreeMap,
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let mut weapons = BTreeMap::new();
    let mut armor = BTreeMap::new();
    let mut rings = BTreeMap::new();

    let mut section = Section::Weapons;
    let mut items = &mut weapons;

    for line in SHOP_ITEMS.lines() {
        if line.is_empty() {
            match section {
                Section::Weapons => {
                    section = Section::Armor;
                    items = &mut armor;
                }
                Section::Armor => {
                    section = Section::Rings;
                    items = &mut rings;
                }
                Section::Rings => {}
            }
            continue;
        }
        if line.contains(':') {
            continue;
        }

        let item = Item::from_str(&line).unwrap();

        items.insert(item.cost, item);
    }

    let mut boss_hp: Option<i32> = None;
    let mut boss_damage: Option<u16> = None;
    let mut boss_armor: Option<u16> = None;

    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        let mut pieces = line.split(": ");
        let label = pieces.next().unwrap();
        match label {
            "Hit Points" => {
                boss_hp = pieces.next().and_then(|v| v.parse().ok());
            }
            "Damage" => {
                boss_damage = pieces.next().and_then(|v| v.parse().ok());
            }
            "Armor" => {
                boss_armor = pieces.next().and_then(|v| v.parse().ok());
            }
            _ => {}
        }
    }

    let state = State {
        player_hp: 100,
        player_weapon: None,
        player_armor: None,
        player_rings: [None; 2],

        boss_hp: boss_hp.unwrap(),
        boss_damage: boss_damage.unwrap(),
        boss_armor: boss_armor.unwrap(),
    };

    let mut part_1 = u16::MAX;
    let mut part_2 = u16::MIN;
    for weapon in weapons.values() {
        let state = State {
            player_weapon: Some(weapon),
            ..state
        };

        if state.clone().play() == Actor::Player {
            part_1 = part_1.min(state.total_item_cost());
        }
        if state.clone().play() == Actor::Boss {
            part_2 = part_2.max(state.total_item_cost());
        }

        for armor_piece in armor.values() {
            let state = State {
                player_armor: Some(armor_piece),
                ..state
            };

            if state.clone().play() == Actor::Player {
                part_1 = part_1.min(state.total_item_cost());
            }
            if state.clone().play() == Actor::Boss {
                part_2 = part_2.max(state.total_item_cost());
            }

            for ring in rings.values() {
                let state = State {
                    player_rings: [Some(ring), None],
                    ..state
                };

                if state.clone().play() == Actor::Player {
                    part_1 = part_1.min(state.total_item_cost());
                }
                if state.clone().play() == Actor::Boss {
                    part_2 = part_2.max(state.total_item_cost());
                }

                for other_ring in rings.values() {
                    if ring == other_ring {
                        continue;
                    }

                    let state = State {
                        player_rings: [Some(ring), Some(other_ring)],
                        ..state
                    };

                    if state.clone().play() == Actor::Player {
                        part_1 = part_1.min(state.total_item_cost());
                    }
                    if state.clone().play() == Actor::Boss {
                        part_2 = part_2.max(state.total_item_cost());
                    }
                }
            }
        }

        for ring in rings.values() {
            let state = State {
                player_rings: [Some(ring), None],
                ..state
            };

            if state.clone().play() == Actor::Player {
                part_1 = part_1.min(state.total_item_cost());
            }
            if state.clone().play() == Actor::Boss {
                part_2 = part_2.max(state.total_item_cost());
            }

            for other_ring in rings.values() {
                if ring == other_ring {
                    continue;
                }

                let state = State {
                    player_rings: [Some(ring), Some(other_ring)],
                    ..state
                };

                if state.clone().play() == Actor::Player {
                    part_1 = part_1.min(state.total_item_cost());
                }
                if state.clone().play() == Actor::Boss {
                    part_2 = part_2.max(state.total_item_cost());
                }
            }
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

#[derive(Debug, Clone)]
struct State<'a> {
    player_hp: i32,
    player_weapon: Option<&'a Item>,
    player_armor: Option<&'a Item>,
    player_rings: [Option<&'a Item>; 2],

    boss_hp: i32,
    boss_damage: u16,
    boss_armor: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Actor {
    Player,
    Boss,
}

impl<'a> State<'a> {
    fn player_items(&self) -> Vec<&Item> {
        let mut items = Vec::with_capacity(4);

        if let Some(weapon) = self.player_weapon {
            items.push(weapon);
        }
        if let Some(armor) = self.player_armor {
            items.push(armor);
        }
        for ring in &self.player_rings {
            if let Some(ring) = ring {
                items.push(ring);
            }
        }

        items
    }

    fn total_item_cost(&self) -> u16 {
        self.player_items()
            .into_iter()
            .fold(0, |acc, item| acc + item.cost)
    }

    fn player_armor(&self) -> u16 {
        self.player_items()
            .into_iter()
            .map(|item| item.armor)
            .sum::<u16>()
    }

    fn player_damage(&self) -> u16 {
        1.max(
            self.player_items()
                .into_iter()
                .map(|item| item.damage)
                .sum::<u16>()
                .checked_sub(self.boss_armor)
                .unwrap_or_default(),
        )
    }

    fn boss_damage(&self) -> u16 {
        1.max(
            self.boss_damage
                .checked_sub(self.player_armor())
                .unwrap_or_default(),
        )
    }

    fn play_round(&mut self) -> Option<Actor> {
        self.boss_hp -= self.player_damage() as i32;
        if self.boss_hp <= 0 {
            return Some(Actor::Player);
        }

        self.player_hp -= self.boss_damage() as i32;
        if self.player_hp <= 0 {
            return Some(Actor::Boss);
        }

        None
    }

    fn play(&mut self) -> Actor {
        loop {
            if let Some(winner) = self.play_round() {
                return winner;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Weapons,
    Armor,
    Rings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    name: String,
    cost: u16,
    damage: u16,
    armor: u16,
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        Ok(Item {
            name: iter.next().ok_or(format!("no weapon: {}", s))?.to_string(),
            cost: iter
                .next()
                .ok_or(format!("no cost: {}", s))?
                .parse()
                .map_err(|e| format!("invalid cost: {} {:?}", s, e))?,
            damage: iter
                .next()
                .ok_or(format!("no damage: {}", s))?
                .parse()
                .map_err(|e| format!("invalid damage: {} {:?}", s, e))?,
            armor: iter
                .next()
                .ok_or(format!("no armor: {}", s))?
                .parse()
                .map_err(|e| format!("invalid armor: {} {:?}", s, e))?,
        })
    }
}

const SHOP_ITEMS: &'static str = "Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage+1     25     1       0
Damage+2     50     2       0
Damage+3    100     3       0
Defense+1    20     0       1
Defense+2    40     0       2
Defense+3    80     0       3";
