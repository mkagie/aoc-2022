//! Command line executable for running part one and part two
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
    let input = parse_input_two(file);
    part_two_internal(input)
}

// TODO -- Update this with the return type
type ReturnType = u64;
type VectorType = Bag;

// Part1
// - Parse per line
// - Cut each line in half and put each half in a HashMap
// - Find the items that are in both
// - Count the scores

struct Bag {
    comp1: HashSet<char>,
    comp2: HashSet<char>,
}
impl Bag {
    fn new(line: &str) -> Self {
        // Get to a list of chars
        let chars = line.chars();
        let mut comp1 = HashSet::new();
        let mut comp2 = HashSet::new();
        let len = chars.clone().count();
        for (i, c) in chars.enumerate() {
            if i < len / 2 {
                comp1.insert(c);
            } else {
                comp2.insert(c);
            }
        }
        Self { comp1, comp2 }
    }

    fn get_score(&self) -> ReturnType {
        let mut sum = 0;
        for val in self.comp1.intersection(&self.comp2) {
            sum += get_priority(val);
        }
        sum
    }
}

fn get_priority(val: &char) -> ReturnType {
    if *val as ReturnType >= 'a' as ReturnType && *val as ReturnType <= 'z' as ReturnType {
        *val as ReturnType - 'a' as ReturnType + 1
    } else {
        *val as ReturnType - 'A' as ReturnType + 27
    }
}

// TODO Implement this
fn parse_input(file: BufReader<File>) -> Vec<VectorType> {
    file.lines()
        .map(|x| Bag::new(x.unwrap().as_str()))
        .collect()
}

fn parse_input_two(file: BufReader<File>) -> Vec<HashSet<char>> {
    file.lines()
        .map(|line| {
            let mut hs = HashSet::new();
            line.unwrap().chars().for_each(|c| {
                let _ = hs.insert(c);
            });
            hs
        })
        .collect()
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    input.iter().map(|bag| bag.get_score()).sum()
}

// TODO Implement this
fn part_two_internal(input: Vec<HashSet<char>>) -> ReturnType {
    input.chunks(3).fold(0, |mut acc, x| {
        // Find overlap in first two
        let mut temp = HashSet::new();
        for y in x[0].intersection(&x[1]) {
            temp.insert(y.to_owned());
        }
        for z in x[2].intersection(&temp) {
            acc += get_priority(z);
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
