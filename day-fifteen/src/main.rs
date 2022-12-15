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
    part_one_internal(input, 2000000)
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
type VectorType = (Sensor, Beacon);
type VectorType2 = u32;

#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}
impl Position {
    /// From "x=<>," "y=<>"
    fn from_xy(x_eq: &str, y_eq: &str) -> Self {
        let x = x_eq
            .replace(",", "")
            .split('=')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let y: i64 = y_eq.split('=').nth(1).unwrap().parse().unwrap();
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, Debug)]
struct Sensor {
    position: Position,
    // TODO(mkagie) Deal with lifetimes instead
    nearest_beacon: Position,
}
impl Sensor {
    fn new(position: Position, nearest_beacon: &Position) -> Self {
        Self {
            position,
            nearest_beacon: nearest_beacon.clone(),
        }
    }

    /// You could not detect a beacon if it is > nearest_distance
    fn could_contain(&self, position: &Position) -> bool {
        let nearest_distance = self.position.manhattan_distance(&self.nearest_beacon);
        self.position.manhattan_distance(position) > nearest_distance
            || position == &self.nearest_beacon
    }
}

#[derive(Clone, Debug)]
struct Beacon(Position);

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    let mut sensor_beacon = input.split(": ");
    // Pull out sensor
    let sensor = sensor_beacon.next().unwrap();
    let mut sensor_xy = sensor.split_whitespace().skip(2);
    let x_eq = sensor_xy.next().unwrap(); // x=<>,
    let y_eq = sensor_xy.next().unwrap(); // y=<>
    let sensor_pos = Position::from_xy(x_eq, y_eq);

    // Pull out beacon
    let beacon = sensor_beacon.next().unwrap();
    let mut beacon_xy = beacon.split_whitespace().skip(4);
    let x_eq = beacon_xy.next().unwrap();
    let y_eq = beacon_xy.next().unwrap();
    let beacon = Beacon(Position::from_xy(x_eq, y_eq));
    let sensor = Sensor::new(sensor_pos, &beacon.0);
    (sensor, beacon)
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType2 {
    todo!()
}

// TODO Implement this
fn part_one_internal(input: Vec<VectorType>, y: i64) -> ReturnType {
    // Determine the number of columns -- min and max x in both beacons and sensors
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    for (sensor, beacon) in input.into_iter() {
        let sensor_x = sensor.position.x;
        let beacon_x = beacon.0.x;

        min_x = min_x.min(sensor_x).min(beacon_x);
        max_x = max_x.max(sensor_x).max(beacon_x);

        sensors.push(sensor);
        beacons.push(beacon);
    }

    // Iterate over values from in to max x and see if it could be picked up
    let mut n_positions = 0;
    for x in min_x..(max_x + 1) {
        // TODO(mkagie) make this 2000000
        let pos = Position { x, y };
        let all_sensors_could_contain = sensors.iter().fold(true, |is_not_contained, sensor| {
            is_not_contained && sensor.could_contain(&pos)
        });
        if !all_sensors_could_contain {
            n_positions += 1;
        }
    }
    n_positions
}

// TODO Implement this
fn part_two_internal(input: Vec<VectorType2>) -> ReturnType {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    }

    #[test]
    fn test_one() {
        let input: Vec<(Sensor, Beacon)> = input().lines().map(|line| map_one(line)).collect();
        println!("Input: {:?}", input);
        assert_eq!(part_one_internal(input, 10), 26);
    }

    #[test]
    fn test_two() {}
}
