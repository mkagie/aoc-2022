//! Command line executable for running part one and part two
use std::{fs::File, io::BufReader, io::BufRead};

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

type ReturnType = u64;

fn main() {
    let args = Args::parse();

    let file = BufReader::new(File::open(args.input_file).expect("Cannot find file"));

    let answer = match args.part {
        Part::Part1 => part_one(file),
        Part::Part2 => part_two(file),
    };

    println!("{:?}", answer);
}

fn parse_input(file: BufReader<File>) -> Vec<Vec<u32>> {
    let input: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
    let mut elves: Vec<Vec<u32>> = Vec::new();
    let mut elf: Vec<u32> = Vec::new();
    for line in input {
        if line == "" {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.parse().unwrap());
        }
    }
    elves

}

fn part_one(file: BufReader<File>) -> ReturnType {
    let elves = parse_input(file);
    part_one_internal(elves)
}

fn part_one_internal(input: Vec<Vec<u32>>) -> ReturnType {
    let val: f32 = input.into_iter().map(|elf| elf.into_iter().map(|x| x as f32).sum()).reduce(|greatest, val| if val > greatest { val } else {greatest}).unwrap();
    val as u64
}

fn part_two(file: BufReader<File>) -> ReturnType {
    let elves = parse_input(file);

    part_two_internal(elves)
}

fn part_two_internal(input: Vec<Vec<u32>>) -> ReturnType {
    let mut calories: Vec<u64> = input.into_iter().map(|elf| elf.into_iter().map(|x| x as u64).sum()).collect();
    calories.sort();
    calories.into_iter().rev().take(3).sum()
}
