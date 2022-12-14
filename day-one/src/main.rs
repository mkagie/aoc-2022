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
    let input = parse_input(file);
    part_one_internal(input)
}

fn part_two(file: BufReader<File>) -> ReturnType {
    let input = parse_input(file);
    part_two_internal(input)
}

// TODO -- Update this with the return type
type ReturnType = u64;
type VectorType = Vec<u64>;

// TODO Implement this
fn parse_input(file: BufReader<File>) -> Vec<VectorType> {
    let input: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
    let mut elves: Vec<VectorType> = Vec::new();
    let mut elf: VectorType = Vec::new();
    for line in input {
        if line.is_empty() {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.parse().unwrap());
        }
    }
    elves
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    input
        .into_iter()
        .map(|elf| elf.into_iter().sum())
        .reduce(|greatest, val| if val > greatest { val } else { greatest })
        .unwrap()
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType>) -> ReturnType {
    let mut calories: VectorType = input.into_iter().map(|elf| elf.into_iter().sum()).collect();
    calories.sort();
    calories.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
