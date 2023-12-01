use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn get_calibration_value(digits: &[&str], line: &str) -> u32 {
    let digit_matches: Vec<_> = digits
        .iter()
        .map(|digit| line.match_indices(digit))
        .flatten()
        .map(|(idx, digit_str)| {
            // array index to number
            let integer = digits.iter().position(|&d| d == digit_str).unwrap() as u32 % 9 + 1;
            (idx, integer)
        })
        .collect();
    let first_digit = digit_matches.iter().min_by_key(|&(idx, _)| idx).unwrap().1;
    let last_digit = digit_matches.iter().max_by_key(|&(idx, _)| idx).unwrap().1;
    first_digit * 10 + last_digit
}

fn main() -> io::Result<()> {
    let raw_calibration_values: Vec<String> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| l.unwrap().parse().unwrap())
    .collect();

    let p1_digits: &[_] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let calibration_sum: u32 = raw_calibration_values
        .iter()
        .map(|l| get_calibration_value(p1_digits, l))
        .sum();
    println!("Calibration value sum: {}", calibration_sum);

    let p2_digits: &[_] = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let calibration_sum_with_words: u32 = raw_calibration_values
        .iter()
        .map(|l| get_calibration_value(p2_digits, l))
        .sum();
    println!("Calibration value sum: {}", calibration_sum_with_words);
    Ok(())
}
