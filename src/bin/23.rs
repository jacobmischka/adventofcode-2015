use std::{
    collections::BTreeMap,
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let mut computer = Computer::new(
        Computer::parse_program(io::stdin().lock().lines().filter_map(Result::ok)).unwrap(),
    );

    let mut computer_1 = computer.clone();
    computer_1.run();

    println!(
        "Part 1: {}",
        computer_1.registers.get(&Register::B).unwrap()
    );

    computer.registers.insert(Register::A, 1);
    computer.run();

    println!("Part 2: {}", computer.registers.get(&Register::B).unwrap());
}

#[derive(Debug, Clone)]
struct Computer {
    pc: isize,
    program: Vec<Instruction>,
    registers: BTreeMap<Register, isize>,
}

impl Computer {
    fn new(program: Vec<Instruction>) -> Computer {
        let mut registers = BTreeMap::new();
        registers.insert(Register::A, 0);
        registers.insert(Register::B, 0);

        Computer {
            pc: 0,
            program,
            registers,
        }
    }

    fn parse_program<I>(iter: I) -> Result<Vec<Instruction>, String>
    where
        I: Iterator<Item = String>,
    {
        iter.map(|s| Instruction::from_str(&s)).collect()
    }

    fn run_step(&mut self) {
        let inst = self.program[self.pc as usize];
        match inst {
            Instruction::Hlf(reg) => {
                *self.registers.entry(reg).or_default() /= 2;
                self.pc += 1;
            }
            Instruction::Tpl(reg) => {
                *self.registers.entry(reg).or_default() *= 3;
                self.pc += 1;
            }
            Instruction::Inc(reg) => {
                *self.registers.entry(reg).or_default() += 1;
                self.pc += 1;
            }
            Instruction::Jmp(offset) => {
                self.pc += offset;
            }
            Instruction::Jie(reg, offset) => {
                if self.registers.get(&reg).unwrap() % 2 == 0 {
                    self.pc += offset;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jio(reg, offset) => {
                if *self.registers.get(&reg).unwrap() == 1 {
                    self.pc += offset;
                } else {
                    self.pc += 1;
                }
            }
        }
    }

    fn run(&mut self) {
        while 0 <= self.pc && (self.pc as usize) < self.program.len() {
            self.run_step();
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        match iter.next().ok_or(format!("missing instruction: {}", s))? {
            "hlf" => Ok(Instruction::Hlf(Register::from_str(
                iter.next().ok_or(format!("missing register: {}", s))?,
            )?)),
            "tpl" => Ok(Instruction::Tpl(Register::from_str(
                iter.next().ok_or(format!("missing register: {}", s))?,
            )?)),
            "inc" => Ok(Instruction::Inc(Register::from_str(
                iter.next().ok_or(format!("missing register: {}", s))?,
            )?)),
            "jmp" => Ok(Instruction::Jmp(
                iter.next()
                    .ok_or(format!("missing offset: {}", s))?
                    .parse()
                    .map_err(|e| format!("invalied offset: {} {:?}", s, e))?,
            )),
            "jie" => Ok(Instruction::Jie(
                Register::from_str(
                    &iter
                        .next()
                        .ok_or(format!("missing register: {}", s))?
                        .replace(',', ""),
                )?,
                iter.next()
                    .ok_or(format!("missing offset: {}", s))?
                    .parse()
                    .map_err(|e| format!("invalied offset: {} {:?}", s, e))?,
            )),
            "jio" => Ok(Instruction::Jio(
                Register::from_str(
                    &iter
                        .next()
                        .ok_or(format!("missing register: {}", s))?
                        .replace(',', ""),
                )?,
                iter.next()
                    .ok_or(format!("missing offset: {}", s))?
                    .parse()
                    .map_err(|e| format!("invalied offset: {} {:?}", s, e))?,
            )),
            s => Err(format!("invalid instruction: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            s => Err(format!("invalid register: {}", s)),
        }
    }
}
