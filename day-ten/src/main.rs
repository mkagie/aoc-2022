//! Command line executable for running part one and part two
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

use ndarray::Array;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short)]
    input_file: String,

    #[command(subcommand)]
    part: Part,
}

#[derive(clap::Subcommand, Debug)]
enum Part {
    Part1,
    Part2,
}

fn main() {
    let args = Args::parse();

    let file = BufReader::new(File::open(args.input_file).expect("Cannot find file"));

    let answer = match args.part {
        Part::Part1 => part_one(file),
        Part::Part2 => part_two(file),
    };

    println!("{:?}", answer);
}

fn part_one(file: BufReader<File>) -> ReturnType {
    let input = parse_input(file, map_one);
    part_one_internal(input)
}

fn part_two(file: BufReader<File>) -> ReturnType {
    let input = parse_input(file, map_two);
    part_two_internal(input)
}

fn parse_input<F, T>(file: BufReader<File>, f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    file.lines().map(|x| f(x.unwrap().as_str())).collect()
}

// TODO -- Update this with the return type
type ReturnType = i64;
type VectorType = Instruction;
type VectorType2 = Instruction;

#[derive(Clone, Debug)]
enum Instruction {
    NoOp,
    AddX { amount: i64, cycle_num: usize },
}
impl Instruction {
    fn from_line(input: &str) -> Self {
        let mut words = input.split_whitespace();
        match words.next().unwrap() {
            "noop" => Self::NoOp,
            "addx" => Self::AddX {
                amount: words.next().unwrap().parse().unwrap(),
                cycle_num: 2,
            },
            _ => panic!("Not valid"),
        }
    }

    fn start_cycle(&mut self) {
        match self {
            Self::NoOp => (),
            Self::AddX { cycle_num, .. } => {
                *cycle_num -= 1;
            }
        }
    }

    fn cycle(self, val: &mut i64) -> Option<Self> {
        match self {
            Self::NoOp => None,
            Self::AddX { amount, cycle_num } => {
                if cycle_num == 0 {
                    *val += amount;
                    None
                } else {
                    Some(self)
                }
            }
        }
    }
}

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    Instruction::from_line(input)
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType2 {
    Instruction::from_line(input)
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    let mut x = 1;
    let mut sum = 0;
    let mut cmd = None;
    let mut cmds = input.into_iter();
    for cycle_num in 1..221 {
        if cmd.is_none() {
            cmd = Some(cmds.next().unwrap());
        }
        let mut loc_cmd = cmd.take().unwrap();
        loc_cmd.start_cycle();
        if [20, 60, 100, 140, 180, 220].contains(&cycle_num) {
            sum += x * cycle_num;
        }
        cmd = loc_cmd.cycle(&mut x);
    }
    sum
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType2>) -> ReturnType {
    let mut arr = Array::ones((6, 40));

    let mut x: i64 = 1;
    let mut cmd = None;
    let mut cmds = input.into_iter();
    for cycle_num in 1..241 {
        if cmd.is_none() {
            cmd = Some(cmds.next().unwrap());
        }
        let mut loc_cmd = cmd.take().unwrap();
        loc_cmd.start_cycle();

        if (x - (cycle_num - 1) % 40).abs() < 2 {
            let center_row = (cycle_num - 1) / 40;
            let center_col = (cycle_num - 1) % 40;
            *arr.get_mut((center_row as usize, center_col as usize))
                .unwrap() = 8;
        }

        cmd = loc_cmd.cycle(&mut x);
    }
    println!("{:?}", arr);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
    }

    #[test]
    fn test_one() {
        let input: Vec<VectorType> = input()
            .lines()
            .map(|line| Instruction::from_line(line))
            .collect();
        let output = part_one_internal(input);
        assert_eq!(output, 13140);
    }

    #[test]
    fn test_two() {}
}
