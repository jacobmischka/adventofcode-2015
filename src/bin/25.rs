use std::{
    collections::{BTreeSet, HashMap},
    io,
    ops::{Deref, DerefMut},
};

const MULTIPLIER: u128 = 252533;
const DIVIDER: u128 = 33554393;

fn main() {
    let mut sheet = Sheet::new();
    sheet.insert((1, 1), 20151125);
    sheet.insert((1, 2), 18749137);
    sheet.insert((1, 3), 17289845);
    sheet.insert((1, 4), 30943339);
    sheet.insert((1, 5), 10071777);
    sheet.insert((1, 6), 33511524);

    sheet.insert((2, 1), 31916031);
    sheet.insert((2, 2), 21629792);
    sheet.insert((2, 3), 16929656);
    sheet.insert((2, 4), 7726640);
    sheet.insert((2, 5), 15514188);
    sheet.insert((2, 6), 4041754);

    sheet.insert((3, 1), 16080970);
    sheet.insert((3, 2), 8057251);
    sheet.insert((3, 3), 1601130);
    sheet.insert((3, 4), 7981243);
    sheet.insert((3, 5), 11661866);
    sheet.insert((3, 6), 16474243);

    sheet.insert((4, 1), 24592653);
    sheet.insert((4, 2), 32451966);
    sheet.insert((4, 3), 21345942);
    sheet.insert((4, 4), 9380097);
    sheet.insert((4, 5), 10600672);
    sheet.insert((4, 6), 31527494);

    sheet.insert((5, 1), 77061);
    sheet.insert((5, 2), 17552253);
    sheet.insert((5, 3), 28094349);
    sheet.insert((5, 4), 6899651);
    sheet.insert((5, 5), 9250759);
    sheet.insert((5, 6), 31663883);

    sheet.insert((6, 1), 33071741);
    sheet.insert((6, 2), 6796745);
    sheet.insert((6, 3), 25397450);
    sheet.insert((6, 4), 24659492);
    sheet.insert((6, 5), 1534922);
    sheet.insert((6, 6), 27995004);

    let row = 2947;
    let col = 3029;

    let mut y = 0;
    'outer: loop {
        y += 1;
        for x in (1..y).rev() {
            sheet.get_val(x, y - x);
            if x == row && y - x == col {
                break 'outer;
            }
        }
    }

    println!("Part 1: {}", sheet.get_val(row, col));
}

#[derive(Debug, Clone)]
struct Sheet(HashMap<(usize, usize), u128>);

impl Sheet {
    fn new() -> Sheet {
        Sheet(HashMap::new())
    }

    fn get_val(&mut self, row: usize, col: usize) -> u128 {
        if let Some(val) = self.get(&(row, col)) {
            *val
        } else {
            let prev = if col == 1 {
                self.get_val(1, row - 1)
            } else {
                self.get_val(row + 1, col - 1)
            };

            let val = (prev * MULTIPLIER) % DIVIDER;
            self.insert((row, col), val);
            val
        }
    }
}

impl Deref for Sheet {
    type Target = HashMap<(usize, usize), u128>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sheet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
