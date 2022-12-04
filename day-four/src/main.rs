//! Command line executable for running part one and part two
use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashSet,
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
type VectorType = Assignment;
type VectorType2 = Assignment;

struct Assignment {
    elf0: HashSet<u64>,
    elf1: HashSet<u64>,
}
impl Assignment {
    fn new(input: &str) -> Self {
        let mut ranges = input.split(",");
        let mut one = ranges.next().unwrap().split("-");
        let start0: u64 = one.next().unwrap().parse().unwrap();
        let end0: u64 = one.next().unwrap().parse().unwrap();
        let mut two = ranges.next().unwrap().split("-");
        let start1: u64 = two.next().unwrap().parse().unwrap();
        let end1: u64 = two.next().unwrap().parse().unwrap();

        let mut elf0 = HashSet::new();
        (start0..=end0).into_iter().for_each(|val| {elf0.insert(val);});
        let mut elf1 = HashSet::new();
        (start1..=end1).into_iter().for_each(|val| {elf1.insert(val);});

        Self {
            elf0, elf1
        }
    }

    fn is_subset(&self) -> bool {
        self.elf0.is_subset(&self.elf1) || self.elf1.is_subset(&self.elf0)
    }

    fn overlap(&self) -> bool {
        !self.elf0.is_disjoint(&self.elf1) && !self.elf1.is_disjoint(&self.elf0)
    }
}

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    Assignment::new(input)
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType2 {
    Assignment::new(input)
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    input.iter().map(|assignment| assignment.is_subset() as ReturnType).sum()
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType2>) -> ReturnType {
    input.iter().map(|assignment| assignment.overlap() as ReturnType).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
