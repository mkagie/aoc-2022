//! Command line executable for running part one and part two
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use clap::Parser;

use id_tree::{InsertBehavior, Node, NodeId, Tree};

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
type VectorType = String;

#[derive(Debug, Clone)]
struct LocalFile {
    size: usize,
    _name: String,
}
impl LocalFile {
    fn try_new(input: &str) -> Result<Self> {
        let mut words = input.split_whitespace();
        let first = words.next().unwrap();
        if first != "dir" {
            let size: usize = first.parse().unwrap();
            let name = words.next().unwrap().to_string();
            Ok(Self { size, _name: name })
        } else {
            Err(anyhow!("Not a file"))
        }
    }
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    files: Vec<LocalFile>,
    pub directory_data_size: usize,
}
impl Directory {
    fn try_new(input: &str) -> Result<Self> {
        let mut words = input.split_whitespace();
        let first = words.next().unwrap();
        if first == "dir" {
            let name = words.next().unwrap().to_string();
            Ok(Self {
                name,
                files: Vec::new(),
                directory_data_size: 0,
            })
        } else {
            Err(anyhow!("Not a file"))
        }
    }

    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
            directory_data_size: 0,
        }
    }

    fn add_file(&mut self, file: LocalFile) {
        self.files.push(file);
    }

    fn get_file_sizes(&self) -> usize {
        self.files.iter().map(|x| x.size).sum()
    }

    fn add_sizes_from_below(&mut self, size: usize) {
        self.directory_data_size += size;
    }

    fn get_full_size(&self) -> usize {
        self.directory_data_size + self.get_file_sizes()
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}
impl Command {
    fn try_new(input: &str) -> Result<Self> {
        let mut words = input.split_whitespace();
        if words.next().unwrap() == "$" {
            let cmd = words.next().unwrap();
            if cmd == "cd" {
                Ok(Command::Cd(words.next().unwrap().to_string()))
            } else {
                Ok(Command::Ls)
            }
        } else {
            Err(anyhow!("Not a command"))
        }
    }
}

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    input.to_string()
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType {
    input.to_string()
}

fn build_tree(input: Vec<VectorType>) -> Tree<Directory> {
    let mut tree = Tree::new();
    // First, create a directory
    let dir = Directory::new("/");
    let mut node_id = tree.insert(Node::new(dir), InsertBehavior::AsRoot).unwrap();
    for line in input {
        // Check to see if this is a command
        if let Ok(cmd) = Command::try_new(&line) {
            match cmd {
                Command::Cd(name) => {
                    if name == "/" {
                        // Do nothing, we did it above
                    } else if name == ".." {
                        // Move backwards
                        node_id = tree.get(&node_id).unwrap().parent().unwrap().clone();
                    } else {
                        // Find the node ID of your children
                        node_id = tree
                            .get(&node_id)
                            .unwrap()
                            .children()
                            .iter()
                            .find(|child| tree.get(child).unwrap().data().name == name)
                            .unwrap()
                            .clone();
                    }
                }
                Command::Ls => {}
            }
        } else if let Ok(dir) = Directory::try_new(&line) {
            // Add it to the tree, do not save node_id
            let _ = tree
                .insert(Node::new(dir), InsertBehavior::UnderNode(&node_id))
                .unwrap();
        } else if let Ok(file) = LocalFile::try_new(&line) {
            // Add the file to the directory
            tree.get_mut(&node_id).unwrap().data_mut().add_file(file);
        }
    }
    tree
}

fn populate_tree(tree: &mut Tree<Directory>) {
    let mut node_ids: Vec<NodeId> = tree
        .traverse_level_order_ids(tree.root_node_id().unwrap())
        .unwrap()
        .collect();
    // Now, make bottom to top
    node_ids.reverse();
    // Traverse and add sizes to the trees
    for node_id in node_ids {
        // Get Node
        let node = tree.get(&node_id).unwrap();
        let dir = node.data();
        // Compute full size of this bad boy
        let size = dir.get_file_sizes() + dir.directory_data_size;
        // Add this to the parent
        let parent_id = if node.parent().is_some() {
            Some(node.parent().unwrap().clone())
        } else {
            None
        };
        if let Some(parent_id) = parent_id {
            tree.get_mut(&parent_id)
                .unwrap()
                .data_mut()
                .add_sizes_from_below(size);
        }
    }
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    let mut tree = build_tree(input);
    populate_tree(&mut tree);
    tree.traverse_level_order(tree.root_node_id().unwrap())
        .unwrap()
        .map(|node| node.data().get_full_size())
        .filter(|&size| size <= 100000)
        .sum()
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType>) -> ReturnType {
    let mut tree = build_tree(input);
    populate_tree(&mut tree);

    // Get total amount of space being used
    let total_free_space = 70000000
        - tree
            .get(tree.root_node_id().unwrap())
            .unwrap()
            .data()
            .get_full_size();
    let needed_space_for_update = 30000000;
    let needed_space = needed_space_for_update - total_free_space;

    tree.traverse_level_order(tree.root_node_id().unwrap())
        .unwrap()
        .map(|node| node.data().get_full_size())
        .filter(|&size| size >= needed_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
