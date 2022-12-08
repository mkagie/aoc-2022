//! Command line executable for running part one and part two
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

use ndarray::{Array, s};

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
type ReturnType = i64;
type VectorType = Vec<i64>;

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    input.chars().map(|c| c.to_digit(10).unwrap() as i64).collect()
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType {
    input.chars().map(|c| c.to_digit(10).unwrap() as i64).collect()
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    let n_rows = input.len();
    let n_cols = input[0].len();
    let arr = Array::from_iter(input.into_iter().flatten()).into_shape((n_rows, n_cols)).unwrap();
    let mut n_visible = 0;
    for row_idx in 0..n_rows {
        for col_idx in 0..n_cols {
            let val = arr.get((row_idx, col_idx)).unwrap().clone();

            // Check the north slice
            let north = arr.slice(s![0..row_idx, col_idx]).iter().max().map(|x| x.clone()).unwrap_or(-1);
            let south = arr.slice(s![row_idx+1.., col_idx]).iter().max().map(|x| x.clone()).unwrap_or(-1);
            let east = arr.slice(s![row_idx, 0..col_idx]).iter().max().map(|x| x.clone()).unwrap_or(-1);
            let west = arr.slice(s![row_idx, col_idx+1..]).iter().max().map(|x| x.clone()).unwrap_or(-1);

            if north < val || south < val || east < val || west < val {
                n_visible += 1;
            }
        }
    }
    n_visible
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType>) -> ReturnType {
    let n_rows = input.len();
    let n_cols = input[0].len();
    let arr = Array::from_iter(input.into_iter().flatten()).into_shape((n_rows, n_cols)).unwrap();
    let mut highest_score = -1;
    for row_idx in 0..n_rows {
        for col_idx in 0..n_cols {
            let val = arr.get((row_idx, col_idx)).unwrap().clone();

            // Check the north slice
            let north_view = arr.slice(s![0..row_idx, col_idx]).iter().rev().enumerate().find_map(|(idx, v)| if v >= &val { Some(idx as ReturnType)} else {None}).unwrap_or(row_idx as i64 - 1) + 1;
            let south_view = arr.slice(s![row_idx + 1.., col_idx]).iter().enumerate().find_map(|(idx, v)| if v >= &val { Some(idx as ReturnType)} else {None}).unwrap_or(n_rows  as i64 - row_idx as i64 - 2) + 1;
            let east_view = arr.slice(s![row_idx, 0..col_idx]).iter().rev().enumerate().find_map(|(idx, v)| if v >= &val { Some(idx as ReturnType)} else {None}).unwrap_or(col_idx as i64 - 1) + 1;
            let west_view = arr.slice(s![row_idx, col_idx + 1..]).iter().enumerate().find_map(|(idx, v)| if v >= &val { Some(idx as ReturnType)} else {None}).unwrap_or(n_cols as i64 - col_idx as i64 - 2) + 1;

            let score = north_view * south_view * east_view * west_view;
            highest_score = score.max(highest_score);

        }
    }
    highest_score
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
