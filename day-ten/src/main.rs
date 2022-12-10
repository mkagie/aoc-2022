//! Command line executable for running part one and part two
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

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
type ReturnType = u64;
type VectorType = Instruction;
type VectorType2 = u32;

enum Instruction {
    NoOp,
    AddX(i64)
}
impl Instruction {
    fn from_line(input: &str) -> Self {
        let mut words = input.split_whitespace();
        match words.next().unwrap() {
            "noop" => Self::NoOp,
            "addx" => Self::AddX(words.next().unwrap().parse().unwrap()),
            _ => panic!("Not valid")
        }
    }
}

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    Instruction::from_line(input)
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType2 {
    todo!()
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    let mut cycle_num = 0;
    let mut X = 1;
    // For each cycle:
    //   - If no current instruction, pull and execute
    //   - If a current instruction, increment
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType2>) -> ReturnType {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
