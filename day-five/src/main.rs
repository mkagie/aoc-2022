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
    part_one_internal(input, get_crates())
}

fn part_two(file: BufReader<File>) -> ReturnType {
    let input = parse_input(file, map_two);
    part_two_internal(input, get_crates())
}

fn parse_input<F, T>(file: BufReader<File>, f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    file.lines().map(|x| f(x.unwrap().as_str())).collect()
}

// TODO -- Update this with the return type
type ReturnType = Vec<char>;
type VectorType = Instruction;


fn get_crates() -> Vec<Vec<char>> {
    // The crates start off as backwards
    let mut crates = vec![
        vec!['G', 'J', 'Z'],
        vec!['C', 'V', 'F', 'W', 'P', 'R', 'L', 'Q'],
        vec!['R', 'G', 'L', 'C', 'M', 'P', 'F'],
        vec!['M', 'H', 'P', 'W', 'B', 'F', 'L'],
        vec!['Q', 'V', 'S', 'F', 'C', 'G'],
        vec!['L', 'T', 'Q', 'M', 'Z', 'J', 'H', 'W'] ,
        vec!['V', 'B', 'S', 'F', 'H'],
        vec!['S', 'Z', 'J', 'F'],
        vec!['T', 'B', 'H', 'F', 'P', 'D', 'C', 'M']
    ];
    // Make them correct
    crates.iter_mut().for_each(|c| c.reverse());
    crates
}

#[derive(Debug)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize
}
impl Instruction {
    fn new(input: &str) -> Self {
        let mut words = input.split_whitespace().skip(1).step_by(2);
        let num = words.next().unwrap().parse().unwrap();
        let from = words.next().unwrap().parse().unwrap();
        let to = words.next().unwrap().parse().unwrap();
        Self {
            num, from, to
        }
    }

    fn apply(&self, crates: &mut[Vec<char>]) {
        for _ in 0..self.num {
            // Grab the last crate from that crate
            let val = crates[self.from-1].pop().unwrap();
            crates[self.to - 1].push(val);
        }
    }

    fn apply_sequential(&self, crates: &mut [Vec<char>]) {
        let mut v = Vec::new();
        for _ in 0..self.num {
            v.push(crates[self.from-1].pop().unwrap());
        }
        v.reverse();
        for val in v {
            crates[self.to-1].push(val);
        }
    }
}

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    Instruction::new(input)
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType {
    Instruction::new(input)
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>, mut crates: Vec<Vec<char>>) -> ReturnType {
    input.iter().for_each(|instruction| instruction.apply(&mut crates));
    crates.iter_mut().map(|c| c.pop().unwrap()).collect()

}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType>, mut crates: Vec<Vec<char>>) -> ReturnType {
    input.iter().for_each(|instruction| instruction.apply_sequential(&mut crates));
    crates.iter_mut().map(|c| c.pop().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        let crates = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P']
        ];

        let input = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let instructions: Vec<VectorType> = input.lines().map(|x| map_one(x)).collect();

        println!("{:?}", part_one_internal(instructions, crates));
    }

    #[test]
    fn test_two() {}
}
