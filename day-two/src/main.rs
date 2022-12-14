//! Command line executable for running part one and part two
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
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
    let input = parse_input(file, Game::from_line);
    part_one_internal(input)
}

fn part_two(file: BufReader<File>) -> ReturnType {
    let input = parse_input(file, Game::from_strategy);
    part_two_internal(input)
}

// TODO -- Update this with the return type
type ReturnType = u64;
type VectorType = Game;

#[derive(Clone)]
enum RPC {
    Rock,
    Paper,
    Scissors,
}
impl RPC {
    fn try_parse(input: &str) -> Result<Self> {
        use RPC::*;
        match input {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => Err(anyhow!("Bad input")),
        }
    }

    fn get_points(&self) -> ReturnType {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn gen_win(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
    fn gen_tie(&self) -> Self {
        self.clone()
    }
    fn gen_loss(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

enum Strategy {
    Win,
    Lose,
    Tie,
}
impl Strategy {
    fn from_xyz(input: &str) -> Self {
        match input {
            "X" => Strategy::Lose,
            "Y" => Strategy::Tie,
            "Z" => Strategy::Win,
            _ => Strategy::Win,
        }
    }
}

struct Game {
    you: RPC,
    opponent: RPC,
}
impl Game {
    fn from_line(input: &str) -> Self {
        let mut inputs = input.split_whitespace();
        let opponent = RPC::try_parse(inputs.next().unwrap()).unwrap();
        let you = RPC::try_parse(inputs.next().unwrap()).unwrap();
        Self { you, opponent }
    }

    fn from_strategy(input: &str) -> Self {
        let mut inputs = input.split_whitespace();

        let opponent = RPC::try_parse(inputs.next().unwrap()).unwrap();
        let strategy = Strategy::from_xyz(inputs.next().unwrap());
        let you = match strategy {
            Strategy::Win => opponent.gen_loss(),
            Strategy::Tie => opponent.gen_tie(),
            Strategy::Lose => opponent.gen_win(),
        };
        Self { you, opponent }
    }

    fn score(&self) -> ReturnType {
        let opposition_score = match self.you {
            RPC::Rock => match self.opponent {
                RPC::Rock => 3,
                RPC::Paper => 0,
                RPC::Scissors => 6,
            },
            RPC::Paper => match self.opponent {
                RPC::Rock => 6,
                RPC::Paper => 3,
                RPC::Scissors => 0,
            },
            RPC::Scissors => match self.opponent {
                RPC::Rock => 0,
                RPC::Paper => 6,
                RPC::Scissors => 3,
            },
        };
        opposition_score + self.you.get_points()
    }
}

// TODO Implement this
fn parse_input<F: Fn(&str) -> Game>(
    file: BufReader<File>,
    generation_function: F,
) -> Vec<VectorType> {
    file.lines()
        .map(|x| generation_function(x.unwrap().as_str()))
        .collect()
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    input.iter().map(|x| x.score()).sum()
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType>) -> ReturnType {
    input.iter().map(|x| x.score()).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
