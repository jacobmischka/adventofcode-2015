use std::io::{self, BufRead};

const WIDTH: usize = 100;
const STEPS: usize = 100;

type Grid = [[bool; WIDTH]; WIDTH];

fn main() {
    let mut start: Grid = [[false; WIDTH]; WIDTH];
    for (row, line) in io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .enumerate()
    {
        for (col, c) in line.char_indices() {
            start[row][col] = match c {
                '#' => true,
                '.' => false,
                _ => panic!("invalid char: {}", c),
            };
        }
    }
    let start = start;

    let mut grid_1 = start.clone();
    let mut grid_2 = fix_corners(start.clone());
    for _ in 0..STEPS {
        grid_1 = run_step(&grid_1);
        grid_2 = fix_corners(run_step(&grid_2));
    }

    println!("Part 1: {}", count_on(&grid_1));
    println!("Part 2: {}", count_on(&grid_2));
}

fn fix_corners(mut grid: Grid) -> Grid {
    grid[0][0] = true;
    grid[0][WIDTH - 1] = true;
    grid[WIDTH - 1][WIDTH - 1] = true;
    grid[WIDTH - 1][0] = true;
    grid
}

fn count_on(grid: &Grid) -> u32 {
    grid.iter().fold(0, |row_count, row| {
        row_count
            + row
                .iter()
                .fold(0, |acc, light_on| if *light_on { acc + 1 } else { acc })
    })
}

fn run_step(current: &Grid) -> Grid {
    let mut new = current.clone();

    for x in 0..WIDTH {
        for y in 0..WIDTH {
            let neighbors = [
                current.get(x + 1).and_then(|row| row.get(y)),
                current.get(x + 1).and_then(|row| row.get(y + 1)),
                current.get(x).and_then(|row| row.get(y + 1)),
                x.checked_sub(1)
                    .and_then(|x| current.get(x).and_then(|row| row.get(y + 1))),
                x.checked_sub(1)
                    .and_then(|x| current.get(x).and_then(|row| row.get(y))),
                x.checked_sub(1).and_then(|x| {
                    y.checked_sub(1)
                        .and_then(|y| current.get(x).and_then(|row| row.get(y)))
                }),
                y.checked_sub(1)
                    .and_then(|y| current.get(x).and_then(|row| row.get(y))),
                y.checked_sub(1)
                    .and_then(|y| current.get(x + 1).and_then(|row| row.get(y))),
            ];

            let num_on = neighbors
                .iter()
                .fold(0, |acc, n| if *n.unwrap_or(&false) { acc + 1 } else { acc });

            if current[x][y] {
                new[x][y] = match num_on {
                    2 | 3 => true,
                    _ => false,
                };
            } else {
                if num_on == 3 {
                    new[x][y] = true;
                }
            }
        }
    }

    new
}
