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
type ReturnType = usize;
type VectorType = Command;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: i64,
    col: i64,
}
impl Position {
    fn compute_tail_adjustment(&self, tail: &mut Self) {
        let row_offset = self.row - tail.row;
        let col_offset = self.col - tail.col;

        if row_offset.abs() <= 1 && col_offset.abs() <= 1 {
            // If touching
        } else if col_offset == 0 {
            // If along row axis
            tail.row += row_offset / 2;
        } else if row_offset == 0 {
            // If along column
            tail.col += col_offset / 2;
        } else {
            // If diagonal
            tail.row += row_offset.signum();
            tail.col += col_offset.signum();
        }
    }
}

enum Direction {
    L,
    R,
    U,
    D,
}
impl Direction {
    fn apply(&self, head: &mut Position) {
        match self {
            Self::L => head.col -= 1,
            Self::R => head.col += 1,
            Self::U => head.row += 1,
            Self::D => head.row -= 1,
        }
    }
}

struct Command {
    dir: Direction,
    amount: usize,
}
impl Command {
    fn from_input(input: &str) -> Self {
        let mut words = input.split_whitespace();
        let dir = match words.next().unwrap() {
            "L" => Direction::L,
            "R" => Direction::R,
            "U" => Direction::U,
            "D" => Direction::D,
            _ => panic!("This bad"),
        };

        let amount = words.next().unwrap().parse().unwrap();

        Self { dir, amount }
    }

    fn apply(&self, head: &mut Position, tail: &mut Position, tail_set: &mut HashSet<Position>) {
        for _ in 0..self.amount {
            self.dir.apply(head);
            head.compute_tail_adjustment(tail);
            tail_set.insert(tail.clone());
        }
    }

    fn apply_to_vec(&self, knots: &mut [&mut Position], tail_set: &mut HashSet<Position>) {
        for _ in 0..self.amount {
            self.dir.apply(knots[0]);
            for idx in 0..knots.len() - 1 {
                let head = knots[idx].clone();
                head.compute_tail_adjustment(knots[idx + 1]);
            }
            tail_set.insert(knots[knots.len() - 1].clone());
        }
    }
}

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    Command::from_input(input)
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType {
    Command::from_input(input)
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    let mut head = Position::default();
    let mut tail = Position::default();
    let mut tail_set = HashSet::new();
    input
        .iter()
        .for_each(|cmd| cmd.apply(&mut head, &mut tail, &mut tail_set));
    tail_set.len()
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType>) -> ReturnType {
    let mut knot0 = Position::default();
    let mut knot1 = Position::default();
    let mut knot2 = Position::default();
    let mut knot3 = Position::default();
    let mut knot4 = Position::default();
    let mut knot5 = Position::default();
    let mut knot6 = Position::default();
    let mut knot7 = Position::default();
    let mut knot8 = Position::default();
    let mut knot9 = Position::default();
    let mut knots = vec![
        &mut knot0, &mut knot1, &mut knot2, &mut knot3, &mut knot4, &mut knot5, &mut knot6,
        &mut knot7, &mut knot8, &mut knot9,
    ];

    let mut tail_set = HashSet::new();
    input
        .iter()
        .for_each(|cmd| cmd.apply_to_vec(&mut knots, &mut tail_set));
    tail_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
    }

    #[test]
    fn test_one() {
        let input: Vec<VectorType> = input()
            .lines()
            .map(|line| Command::from_input(line))
            .collect();
        assert_eq!(part_one_internal(input), 13);
    }

    #[test]
    fn test_two() {}
}
