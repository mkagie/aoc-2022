//! Command line executable for running part one and part two
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use slab_tree::NodeRef;

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

fn parse_input(file: BufReader<File>) -> Vec<[Tree; 2]> {
    let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
    lines
        .chunks(3)
        .map(|chunk| [Tree::new(chunk[0].as_str()), Tree::new(chunk[1].as_str())])
        .collect()
}

// TODO -- Update this with the return type
type ReturnType = usize;
type VectorType = [Tree; 2];

#[derive(Debug)]
enum Node {
    Integer(usize),
    List,
}

#[derive(Debug)]
struct Tree(slab_tree::Tree<Node>);
impl Tree {
    fn new(input: &str) -> Self {
        let mut t = slab_tree::Tree::new();
        let mut current_node_idx = None;
        let mut chars = input.chars().peekable();
        while chars.peek().is_some() {
            let mut c = chars.next().unwrap();
            if c == '[' {
                // Start a new list node
                let node = Node::List;
                if let Some(idx) = current_node_idx {
                    let mut current_node = t.get_mut(idx).unwrap();
                    let new_node = current_node.append(node);
                    current_node_idx = Some(new_node.node_id());
                } else {
                    let new_node = t.set_root(node);
                    current_node_idx = Some(new_node);
                }
            } else if c == ']' {
                // This list has ended
                current_node_idx = t
                    .get(current_node_idx.take().unwrap())
                    .unwrap()
                    .parent()
                    .map(|node| node.node_id());
            } else {
                // Parse the string to an integer
                let mut integer_str = String::new();
                while c != ',' && c != ']' {
                    integer_str.push_str(c.to_string().as_str());
                    c = chars.next().unwrap();
                }
                if !integer_str.is_empty() {
                    let node = Node::Integer(integer_str.parse().unwrap());
                    // Add to the current node as a child
                    t.get_mut(current_node_idx.clone().unwrap())
                        .unwrap()
                        .append(node);
                }
            }
        }
        Self(t)
    }

    fn is_in_order(&self, other: &Self) -> bool {
        // Looks like it is a breadth-first search across the two trees
        let left = self.0.root().unwrap();
        let right = other.0.root().unwrap();
        compare_lr(left, right)
    }
}

fn compare_lr(mut left: NodeRef<Node>, mut right: NodeRef<Node>) -> bool {
    match (left.data(), right.data()) {
        (Node::Integer(i0), Node::Integer(i1)) => {
            if i0 < i1 {
                // Check the sibling for both
                // If left runs out of siblings first, they are in the right order
                let left = left.next_sibling();
                let right = right.next_sibling();
                match (left, right) {
                    (None, Some(_)) => true,
                    (Some(_), None) => false,
                    (Some(left), Some(right)) => compare_lr(left, right),
                    (None, None) => true,
                }
            } else {
                return false;
            }
        }
        (Node::List, Node::List) => {
            // Compare each child, left to right
            match (left.first_child(), right.first_child()) {
                (Some(left), Some(right)) => compare_lr(left, right),
                (Some(_), None) => false,
                (None, Some(_)) => true,
                (None, None) => true,
            }
        }
        (Node::Integer(_), Node::List) => {
            // Convert the integer to a list === convert the list to the next integer
            if let Some(right) = right.first_child() {
                compare_lr(left, right)
            } else {
                // The list is empty, which means that right will be out of siblings first
                // This means return false
                false
            }
        }
        (Node::List, Node::Integer(_)) => {
            // Convert the integer to a list === convert the list to the next integer
            if let Some(left) = left.first_child() {
                compare_lr(left, right)
            } else {
                // The list is empty. As such, the left has run out of siblings first
                true
            }
        }
    }
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    input
        .into_iter()
        .enumerate()
        .map(|(pair_idx, [tree1, tree2])| {
            if tree1.is_in_order(&tree2) {
                Some(pair_idx + 1)
            } else {
                None
            }
        })
        .flatten()
        .sum()
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType>) -> ReturnType {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]

"
    }

    #[test]
    fn test_one() {
        let lines: Vec<&str> = input().lines().collect();
        let input: Vec<VectorType> = lines
            .chunks(3)
            .map(|chunks| [Tree::new(chunks[0]), Tree::new(chunks[1])])
            .collect();
        assert_eq!(part_one_internal(input), 13);
    }

    #[test]
    fn test_two() {}
}
