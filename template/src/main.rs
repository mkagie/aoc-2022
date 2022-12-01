//! Command line executable for running part one and part two
use std::{fs::File, io::BufReader};

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
        Part::Part1 => part_one(&file),
        Part::Part2 => part_two(&file),
    };

    println!("{:?}", answer);
}

fn part_one<T>(file: &BufReader<T>) {}

fn part_two<T>(file: &BufReader<T>) {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
