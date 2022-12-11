//! Command line executable for running part one and part two
use std::{
    cell::RefCell,
    collections::VecDeque,
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
    let input = parse_input(file);
    part_one_internal(input)
}

fn part_two(file: BufReader<File>) -> ReturnType {
    let input = parse_input(file);
    part_two_internal(input)
}

fn parse_input(file: BufReader<File>) -> Vec<Monkey> {
    let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
    lines
        .chunks(7)
        .map(|lines| Monkey::from_lines(lines.to_owned()))
        .collect()
}

// TODO -- Update this with the return type
type ReturnType = usize;
type VectorType = Monkey;
type VectorType2 = Monkey;

struct Monkey {
    items: VecDeque<usize>,
    operation: Box<dyn FnMut(usize) -> usize>,
    test: Box<dyn FnMut(usize) -> bool>,
    monkey_throw_idxs: (usize, usize),
    n_items_counted: usize,
}
impl Monkey {
    /// Take the whole monkey definition
    fn from_lines(input: Vec<String>) -> Self {
        // Example:
        // Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3

        // First line doesn't matter
        let mut lines = input.into_iter().skip(1);

        // Get starting items
        let starting_items = lines.next().unwrap();
        let items: VecDeque<usize> = starting_items.split(":").collect::<Vec<&str>>()[1]
            .split(",")
            .map(|x| x.trim().parse().unwrap())
            .collect();

        // Parse operation
        let operation_line = lines.next().unwrap();
        let operation_parts = operation_line.split('=').collect::<Vec<&str>>()[1]
            .split_whitespace()
            .collect::<Vec<&str>>();
        let inline_func = match operation_parts[1] {
            "*" => |old, val| old * val,
            "+" => |old, val| old + val,
            "-" => |old, val| old - val,
            _ => panic!("Bad sign: {:?}", operation_parts[1]),
        };
        let second_number = operation_parts[2].parse::<usize>();
        let operation: Box<dyn FnMut(usize) -> usize> = match second_number {
            Ok(val) => Box::new(move |old| inline_func(old, val)),
            Err(_) => Box::new(move |old| inline_func(old, old)),
        };

        // Test
        let test_denominator: usize = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(3)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let test: Box<dyn FnMut(usize) -> bool> = Box::new(move |val| val % test_denominator == 0);

        // Monkey throw indeces
        let true_idx: usize = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let false_idx: usize = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Self {
            items,
            operation,
            test,
            monkey_throw_idxs: (true_idx, false_idx),
            n_items_counted: 0,
        }
    }
}

// TODO Implement this
fn part_one_internal(monkeys: Vec<VectorType>) -> ReturnType {
    // let mut mut_ref_monkeys: Vec<&mut Monkey> = monkeys.iter_mut().collect();
    let monkeys: Vec<RefCell<Monkey>> = monkeys.into_iter().map(|x| RefCell::new(x)).collect();

    for _round in 0..20 {
        // let mut_ref_monkeys: Vec<&mut Monkey> = monkeys.iter_mut().collect();
        // for idx in 0..n_monkeys {
        for monkey in monkeys.iter() {
            let mut monkey = monkey.borrow_mut();
            while !monkey.items.is_empty() {
                let item = monkey.items.pop_front().unwrap();
                let worry_level = (monkey.operation)(item) / 3;
                let monkey2 = if (monkey.test)(worry_level) {
                    monkeys.get(monkey.monkey_throw_idxs.0).unwrap()
                } else {
                    monkeys.get(monkey.monkey_throw_idxs.1).unwrap()
                };
                monkey2.borrow_mut().items.push_back(worry_level);

                monkey.n_items_counted += 1;
            }
        }
    }

    let mut inspected_items: Vec<usize> =
        monkeys.iter().map(|x| x.borrow().n_items_counted).collect();
    inspected_items.sort();
    inspected_items.reverse();
    inspected_items[0] * inspected_items[1]
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType2>) -> ReturnType {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}

    #[test]
    fn test_two() {}
}
