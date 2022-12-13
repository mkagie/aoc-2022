//! Command line executable for running part one and part two
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

use petgraph::{
    algo::{astar, dijkstra},
    graph::Graph,
};

use rayon::prelude::*;

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
type VectorType = Vec<usize>;

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    input.chars().map(|c| convert_letter_to_number(c)).collect()
}

fn convert_letter_to_number(c: char) -> usize {
    if c >= 'a' && c <= 'z' {
        c as usize - 'a' as usize + 1
    } else if c == 'S' {
        0
        // convert_letter_to_number('a')
    } else if c == 'E' {
        // convert_letter_to_number('z')
        27
    } else {
        panic!("Not an acceptable input");
    }
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType {
    input.chars().map(|c| convert_letter_to_number(c)).collect()
}

// Node is:
// - value in graph
// - Index in graph
//
// Edge is:
// - 1 if left, right, up, down and within 1
// - 99 if not

// We can construct an adjacency matrix

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    // Build the graph
    let mut g = Graph::<usize, usize>::new();
    let mut nodes = Vec::new();
    let n_rows = input.len();
    let n_cols = input[0].len();

    // Find Start -- 0
    let start_idx = input
        .iter()
        .enumerate()
        .map(|(row_idx, row)| (row_idx, row.iter().position(|x| *x == 0)))
        .filter(|(_, col_idx)| col_idx.is_some())
        .map(|(row_idx, col_idx)| (row_idx, col_idx.unwrap()))
        .next()
        .unwrap();
    let end_idx = input
        .iter()
        .enumerate()
        .map(|(row_idx, row)| (row_idx, row.iter().position(|x| *x == 27)))
        .filter(|(_, col_idx)| col_idx.is_some())
        .map(|(row_idx, col_idx)| (row_idx, col_idx.unwrap()))
        .next()
        .unwrap();

    // Create nodes
    for row_idx in 0..n_rows {
        let mut row_vec = Vec::new();
        for col_idx in 0..n_cols {
            let node = input[row_idx][col_idx];
            row_vec.push(g.add_node(node));
        }
        nodes.push(row_vec);
    }

    // Create edges
    for row_idx in 0..n_rows {
        for col_idx in 0..n_cols {
            let node_idx = nodes[row_idx][col_idx];
            let node_weight = g.node_weight(node_idx).unwrap().clone();
            // Try N
            if row_idx >= 1 {
                if let Some(north_idx) = nodes.get(row_idx - 1).and_then(|row| row.get(col_idx)) {
                    let north_weight = g.node_weight(*north_idx).unwrap();
                    if check_nodes(node_weight as i64, *north_weight as i64) {
                        // Add a path
                        g.update_edge(node_idx, *north_idx, 1);
                    }
                }
            }
            // Try S
            if let Some(south_idx) = nodes.get(row_idx + 1).and_then(|row| row.get(col_idx)) {
                let south_weight = g.node_weight(*south_idx).unwrap();
                if check_nodes(node_weight as i64, *south_weight as i64) {
                    // Add a path
                    g.update_edge(node_idx, *south_idx, 1);
                }
            }
            // Try E
            if col_idx >= 1 {
                if let Some(east_idx) = nodes.get(row_idx).and_then(|row| row.get(col_idx - 1)) {
                    let east_weight = g.node_weight(*east_idx).unwrap();
                    if check_nodes(node_weight as i64, *east_weight as i64) {
                        // Add a path
                        g.update_edge(node_idx, *east_idx, 1);
                    }
                }
            }
            // Try W
            if let Some(west_idx) = nodes.get(row_idx).and_then(|row| row.get(col_idx + 1)) {
                let west_weight = g.node_weight(*west_idx).unwrap();
                if check_nodes(node_weight as i64, *west_weight as i64) {
                    // Add a path
                    g.update_edge(node_idx, *west_idx, 1);
                }
            }
        }
    }
    let x = dijkstra(
        &g,
        nodes[start_idx.0][start_idx.1],
        Some(nodes[end_idx.0][end_idx.1]),
        |edge| *edge.weight(),
    );
    *x.get(&nodes[end_idx.0][end_idx.1]).unwrap()
}

fn check_nodes(current: i64, next: i64) -> bool {
    next - current <= 1
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType>) -> ReturnType {
    // Build the graph
    let mut g = Graph::<usize, usize>::new();
    let mut nodes = Vec::new();
    let n_rows = input.len();
    let n_cols = input[0].len();

    // Find Start -- 0
    let start_idx = input
        .iter()
        .enumerate()
        .map(|(row_idx, row)| (row_idx, row.iter().position(|x| *x == 0)))
        .filter(|(_, col_idx)| col_idx.is_some())
        .map(|(row_idx, col_idx)| (row_idx, col_idx.unwrap()))
        .next()
        .unwrap();
    let end_idx = input
        .iter()
        .enumerate()
        .map(|(row_idx, row)| (row_idx, row.iter().position(|x| *x == 27)))
        .filter(|(_, col_idx)| col_idx.is_some())
        .map(|(row_idx, col_idx)| (row_idx, col_idx.unwrap()))
        .next()
        .unwrap();

    // Create nodes
    for row_idx in 0..n_rows {
        let mut row_vec = Vec::new();
        for col_idx in 0..n_cols {
            let node = input[row_idx][col_idx];
            row_vec.push(g.add_node(node));
        }
        nodes.push(row_vec);
    }

    // Create edges
    for row_idx in 0..n_rows {
        for col_idx in 0..n_cols {
            let node_idx = nodes[row_idx][col_idx];
            let node_weight = g.node_weight(node_idx).unwrap().clone();
            // Try N
            if row_idx >= 1 {
                if let Some(north_idx) = nodes.get(row_idx - 1).and_then(|row| row.get(col_idx)) {
                    let north_weight = g.node_weight(*north_idx).unwrap();
                    if check_nodes(node_weight as i64, *north_weight as i64) {
                        // Add a path
                        g.update_edge(node_idx, *north_idx, 1);
                    }
                }
            }
            // Try S
            if let Some(south_idx) = nodes.get(row_idx + 1).and_then(|row| row.get(col_idx)) {
                let south_weight = g.node_weight(*south_idx).unwrap();
                if check_nodes(node_weight as i64, *south_weight as i64) {
                    // Add a path
                    g.update_edge(node_idx, *south_idx, 1);
                }
            }
            // Try E
            if col_idx >= 1 {
                if let Some(east_idx) = nodes.get(row_idx).and_then(|row| row.get(col_idx - 1)) {
                    let east_weight = g.node_weight(*east_idx).unwrap();
                    if check_nodes(node_weight as i64, *east_weight as i64) {
                        // Add a path
                        g.update_edge(node_idx, *east_idx, 1);
                    }
                }
            }
            // Try W
            if let Some(west_idx) = nodes.get(row_idx).and_then(|row| row.get(col_idx + 1)) {
                let west_weight = g.node_weight(*west_idx).unwrap();
                if check_nodes(node_weight as i64, *west_weight as i64) {
                    // Add a path
                    g.update_edge(node_idx, *west_idx, 1);
                }
            }
        }
    }
    let end_node = nodes[end_idx.0][end_idx.1];
    nodes
        .par_iter()
        .enumerate()
        .map(|(row_idx, node_row)| {
            node_row
                .iter()
                .enumerate()
                .map(|(col_idx, node)| {
                    // println!("Trying: {:?}, {:?}", row_idx, col_idx);
                    let val = if *g.node_weight(*node).unwrap() == 1 {
                        astar(
                            &g,
                            *node,
                            |n| n == end_node,
                            |edge| *edge.weight(),
                            |node| {
                                // Find the location of the node
                                let idx = nodes
                                    .iter()
                                    .enumerate()
                                    .map(|(row_idx, row)| {
                                        (row_idx, row.iter().position(|x| *x == node))
                                    })
                                    .filter(|(_, col_idx)| col_idx.is_some())
                                    .map(|(row_idx, col_idx)| (row_idx, col_idx.unwrap()))
                                    .next()
                                    .unwrap();
                                usize::try_from(
                                    (row_idx as i64 - idx.0 as i64).abs()
                                        + (col_idx as i64 - idx.1 as i64).abs(),
                                )
                                .unwrap()
                            },
                        )
                        .and_then(|(steps, _)| Some(steps))
                    } else {
                        None
                    };
                    val
                })
                .flatten()
                .min()
                .unwrap_or(usize::MAX)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
    }

    #[test]
    fn test_one() {
        let input = input();
        let input: Vec<VectorType> = input.lines().map(|line| map_one(line)).collect();
        assert_eq!(part_one_internal(input), 31);
    }

    #[test]
    fn test_two() {
        let input = input();
        let input: Vec<VectorType> = input.lines().map(|line| map_two(line)).collect();
        assert_eq!(part_two_internal(input), 29);
    }
}
