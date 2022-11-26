//! Useful utils to read in AoC files and convert them into data structures to manipulate.

use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

/// Read in a file and represent it as a vector of Strings
pub fn file_to_string_vec(filename: &str) -> Vec<String> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect()
}

/// Read in a file and parse as a vector of numbers
pub fn file_to_num_vec<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    file_to_string_vec(filename)
        .iter()
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

/// Read in a file and represent as a vector of characters
pub fn file_chars_to_int_vec<T>(filename: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    file_to_string_vec(filename)
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| c.to_string().parse::<T>().unwrap())
                .collect()
        })
        .collect()
}

/// Split a string into a vector of words
pub fn split_by_whitespace(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}

// TODO(mkagie) Probably can remove this
/// Convert an array of str to a vector of Strings
pub fn str_array_to_vec(input: &[&str]) -> Vec<String> {
    input.iter().map(|x| x.to_string()).collect()
}

/// Convert an array of strings to an array of numbers
pub fn str_array_to_int_vec(input: &[&str]) -> Vec<i32> {
    input.iter().map(|x| x.parse::<i32>().unwrap()).collect()
}

/// Convert lines to a vector of strings
pub fn str_to_string_vec(input: &str) -> Vec<String> {
    input.lines().into_iter().map(|x| x.to_string()).collect()
}

/// Convert a file to vector of vector of characters
pub fn str_chars_to_int_vec<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    str_to_string_vec(input)
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| c.to_string().parse::<T>().unwrap())
                .collect()
        })
        .collect()
}
