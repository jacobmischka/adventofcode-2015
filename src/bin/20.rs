use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u128 = input.parse().unwrap();

    let mut part_1 = None;
    let mut part_2 = None;
    let mut house = 0;
    loop {
        house += 1;

        if part_1.is_none() {
            let mut presents = 0;
            for i in 1..=(house as f64).sqrt().ceil() as u128 {
                if house % i == 0 {
                    presents += i * 10;
                    presents += (house / i) * 10;
                }
            }
            if presents >= input {
                part_1 = Some(house);
            }
        }

        if part_2.is_none() {
            let mut presents = 0;
            for i in 1..=50u128.min((house as f64).sqrt().ceil() as u128) {
                if house % i == 0 {
                    presents += (house / i) * 11;
                }
            }
            if presents >= input {
                part_2 = Some(house);
            }
        }

        if part_1.is_some() && part_2.is_some() {
            break;
        }
    }

    println!("Part 1: {}", part_1.unwrap());
    println!("Part 2: {}", part_2.unwrap());
}
