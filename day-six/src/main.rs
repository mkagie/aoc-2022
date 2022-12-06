//! Command line executable for running part one and part two
use anyhow::{anyhow, Result};
use std::{
    collections::HashSet,
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

fn parse_input(file: BufReader<File>) -> String {
    file.lines().next().unwrap().unwrap()
}

// TODO -- Update this with the return type
type ReturnType = usize;

// TODO Implement this
fn part_one_internal(input: String) -> ReturnType {
    find_first_unique(input, 4).unwrap()
}

fn part_two_internal(input: String) -> ReturnType {
    find_first_unique(input, 14).unwrap()
}

fn find_first_unique(input: String, window_size: usize) -> Result<ReturnType> {
    let chars: Vec<char> = input.chars().collect();

    for (idx, window) in chars.windows(window_size).enumerate() {
        if is_all_unique(window) {
            return Ok(idx + window_size);
        }
    }
    Err(anyhow!("Could not find unique"))
}

fn is_all_unique(input: &[char]) -> bool {
    let mut hash = HashSet::new();
    for c in input {
        if !hash.insert(c) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
