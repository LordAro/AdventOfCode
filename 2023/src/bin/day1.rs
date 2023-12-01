use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn get_calibration_value(digits: &[&str], line: &str) -> usize {
    let digit_matches: Vec<_> = digits
        .iter()
        .enumerate()
        .flat_map(|(digit_idx, digit_str)| {
            // We only need the specific digit index (which we convert to the actual number), so drop the string
            line.match_indices(digit_str)
                .map(move |(line_idx, _)| (line_idx, digit_idx % 9 + 1))
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
