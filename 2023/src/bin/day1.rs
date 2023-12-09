use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn find_any_digit(digits: &[&str], slice: &str) -> Option<usize> {
    digits
        .iter()
        .enumerate()
        .find(|(_, &digit_str)| slice.starts_with(digit_str))
        .map(|(idx, _)| idx % 9 + 1)
}

fn get_calibration_value(digits: &[&str], line: &str) -> usize {
    let first_digit = (0..line.len())
        .find_map(|idx| find_any_digit(digits, &line[idx..]))
        .unwrap();
    let last_digit = (0..line.len())
        .rev()
        .find_map(|idx| find_any_digit(digits, &line[idx..]))
        .unwrap();
    first_digit * 10 + last_digit
}

fn main() -> io::Result<()> {
    let raw_calibration_values: Vec<_> = BufReader::new(File::open(
        env::args().nth(1).expect("Incorrect number of arguments"),
    )?)
    .lines()
    .map(|l| l.unwrap())
    .collect();

    let p1_digits: &[_] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let calibration_sum: usize = raw_calibration_values
        .iter()
        .map(|l| get_calibration_value(p1_digits, l))
        .sum();
    println!("Calibration value sum: {}", calibration_sum);

    let p2_digits: &[_] = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let calibration_sum_with_words: usize = raw_calibration_values
        .iter()
        .map(|l| get_calibration_value(p2_digits, l))
        .sum();
    println!("Calibration value sum: {}", calibration_sum_with_words);
    Ok(())
}
