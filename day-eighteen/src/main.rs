//! Command line executable for running part one and part two
use std::{
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
type VectorType = Droplet;
type VectorType2 = Droplet;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Droplet {
    x: i64,
    y: i64,
    z: i64,
    faces: Faces,
}
impl Droplet {
    fn from_line(input: &str) -> Self {
        let mut numbers = input
            .split(',')
            .map(|number| number.parse::<i64>().unwrap());
        Self {
            x: numbers.next().unwrap(),
            y: numbers.next().unwrap(),
            z: numbers.next().unwrap(),
            faces: Faces::default(),
        }
    }

    // In-place checking if the other inputs are blocking
    fn check_if_is_blocking(&mut self, other: &Droplet) {
        // If Manhattan distance == 1
        if (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs() == 1 {
            if (self.x - other.x).abs() == 1 {
                if self.x - other.x > 0 {
                    // Positive X
                    self.faces.px.mark_blocked();
                } else {
                    self.faces.nx.mark_blocked();
                }
            } else if (self.y - other.y).abs() == 1 {
                if self.y - other.y > 0 {
                    self.faces.py.mark_blocked();
                } else {
                    self.faces.ny.mark_blocked()
                }
            } else if self.z - other.z > 0 {
                self.faces.pz.mark_blocked();
            } else {
                self.faces.nz.mark_blocked();
            }
        }
    }

    /// Check if the unblocked faces are interior
    ///
    /// This assumes that you have already checked if is blocking
    fn check_if_unblocking_is_interior(
        &mut self,
        min_x: i64,
        max_x: i64,
        min_y: i64,
        max_y: i64,
        min_z: i64,
        max_z: i64,
    ) {
        if matches!(
            self.faces.px,
            BlockingStatus::UnblockedInterior | BlockingStatus::UnblockedExterior
        ) && self.x > min_x
            && self.x < max_x
        {
            println!("Marking {:?} px as interior", self);
            self.faces.px.mark_unblocked_interior();
        }
        if matches!(
            self.faces.nx,
            BlockingStatus::UnblockedInterior | BlockingStatus::UnblockedExterior
        ) && self.x > min_x
            && self.x < max_x
        {
            println!("Marking {:?} nx as interior", self);
            self.faces.nx.mark_unblocked_interior();
        }
        if matches!(
            self.faces.py,
            BlockingStatus::UnblockedInterior | BlockingStatus::UnblockedExterior
        ) && self.y > min_y
            && self.y < max_y
        {
            println!("Marking {:?} py as interior", self);
            self.faces.py.mark_unblocked_interior();
        }
        if matches!(
            self.faces.ny,
            BlockingStatus::UnblockedInterior | BlockingStatus::UnblockedExterior
        ) && self.y > min_y
            && self.y < max_y
        {
            println!("Marking {:?} ny as interior", self);
            self.faces.ny.mark_unblocked_interior();
        }
        if matches!(
            self.faces.pz,
            BlockingStatus::UnblockedInterior | BlockingStatus::UnblockedExterior
        ) && self.z > min_z
            && self.z < max_z
        {
            println!("Marking {:?} pz as interior", self);
            self.faces.pz.mark_unblocked_interior();
        }
        if matches!(
            self.faces.nz,
            BlockingStatus::UnblockedInterior | BlockingStatus::UnblockedExterior
        ) && self.z > min_z
            && self.z < max_z
        {
            println!("Marking {:?} nz as interior", self);
            self.faces.nz.mark_unblocked_interior();
        }
    }

    fn count_unblocked(&self) -> usize {
        self.faces.count_unblocked()
    }

    fn count_unblocked_exterior(&self) -> usize {
        self.faces.count_unblocked_exterior()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Faces {
    px: BlockingStatus,
    nx: BlockingStatus,
    py: BlockingStatus,
    ny: BlockingStatus,
    pz: BlockingStatus,
    nz: BlockingStatus,
}
impl Faces {
    fn count_unblocked(&self) -> usize {
        matches!(
            self.px,
            BlockingStatus::UnblockedExterior | BlockingStatus::UnblockedInterior
        ) as usize
            + matches!(
                self.nx,
                BlockingStatus::UnblockedExterior | BlockingStatus::UnblockedInterior
            ) as usize
            + matches!(
                self.py,
                BlockingStatus::UnblockedExterior | BlockingStatus::UnblockedInterior
            ) as usize
            + matches!(
                self.ny,
                BlockingStatus::UnblockedExterior | BlockingStatus::UnblockedInterior
            ) as usize
            + matches!(
                self.pz,
                BlockingStatus::UnblockedExterior | BlockingStatus::UnblockedInterior
            ) as usize
            + matches!(
                self.nz,
                BlockingStatus::UnblockedExterior | BlockingStatus::UnblockedInterior
            ) as usize
    }

    fn count_unblocked_exterior(&self) -> usize {
        matches!(self.px, BlockingStatus::UnblockedExterior) as usize
            + matches!(self.nx, BlockingStatus::UnblockedExterior) as usize
            + matches!(self.py, BlockingStatus::UnblockedExterior) as usize
            + matches!(self.ny, BlockingStatus::UnblockedExterior) as usize
            + matches!(self.pz, BlockingStatus::UnblockedExterior) as usize
            + matches!(self.nz, BlockingStatus::UnblockedExterior) as usize
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BlockingStatus {
    Blocked,
    UnblockedExterior,
    UnblockedInterior,
}
impl Default for BlockingStatus {
    fn default() -> Self {
        Self::UnblockedExterior
    }
}
impl BlockingStatus {
    fn mark_blocked(&mut self) {
        *self = Self::Blocked;
    }

    fn mark_unblocked_interior(&mut self) {
        *self = Self::UnblockedInterior;
    }
}

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    Droplet::from_line(input)
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType2 {
    Droplet::from_line(input)
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    // For each droplet, check every other droplet to see what sides are blocked
    // count_unblocked the number of sides blocked
    let mut droplets = input;
    let droplets_two = droplets.clone();
    droplets
        .iter_mut()
        .map(|droplet| {
            for d in droplets_two.iter() {
                if d != droplet {
                    droplet.check_if_is_blocking(d);
                }
            }
            droplet.count_unblocked()
        })
        .sum()
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType2>) -> ReturnType {
    let mut droplets = input;
    let droplets_two = droplets.clone();
    let min_x = droplets.iter().map(|droplet| droplet.x).min().unwrap();
    let max_x = droplets.iter().map(|droplet| droplet.x).max().unwrap();
    let min_y = droplets.iter().map(|droplet| droplet.y).min().unwrap();
    let max_y = droplets.iter().map(|droplet| droplet.y).max().unwrap();
    let min_z = droplets.iter().map(|droplet| droplet.z).min().unwrap();
    let max_z = droplets.iter().map(|droplet| droplet.z).max().unwrap();
    println!(
        "{:?} - {:?}\t{:?} - {:?}\t{:?} - {:?}",
        min_x, max_x, min_y, max_y, min_z, max_z
    );
    droplets
        .iter_mut()
        .map(|droplet| {
            for d in droplets_two.iter() {
                if d != droplet {
                    droplet.check_if_is_blocking(d);
                }
            }
            droplet.check_if_unblocking_is_interior(min_x, max_x, min_y, max_y, min_z, max_z);
            droplet.count_unblocked_exterior()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
    }

    #[test]
    fn test_one() {
        let input = input();
        let droplets: Vec<VectorType> = input.lines().map(|line| map_one(line)).collect();

        assert_eq!(part_one_internal(droplets), 64);
    }

    #[test]
    fn test_two() {
        let input = input();
        let droplets: Vec<VectorType> = input.lines().map(|line| map_two(line)).collect();

        assert_eq!(part_two_internal(droplets), 58);
    }
}
