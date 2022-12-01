//! Command line executable for running part one and part two
use std::{fs::File, io::{BufReader, BufRead}};

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
type VectorType = u32;


// TODO Implement this
fn parse_input(file: BufReader<File>) -> Vec<VectorType> {
    let input: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
    todo!()
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    todo!()
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType>) -> ReturnType {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
