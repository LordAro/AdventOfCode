use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let raw_calibration_values: Vec<String> =
        BufReader::new(File::open(env::args().nth(1).unwrap()).expect("Could not open input file"))
            .lines()
            .map(|l| l.unwrap().parse().unwrap())
            .collect();

    let raw_calibration_values: Vec<&str> = vec![
        //        "1abc2",
        //        "pqr3stu8vwx",
        //        "a1b2c3d4e5f",
        //        "treb7uchet",
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    //let calibration_values: Vec<_> = raw_calibration_values
    //    .iter()
    //    .map(|r| {
    //        r.chars()
    //            .find(|c| c.is_digit(10))
    //            .unwrap()
    //            .to_digit(10)
    //            .unwrap()
    //            * 10
    //            + r.chars()
    //                .rfind(|c| c.is_digit(10))
    //                .unwrap()
    //                .to_digit(10)
    //                .unwrap()
    //    })
    //    .collect();
    //let calibration_sum: u32 = calibration_values.iter().sum();
    //println!("Calibration value sum: {}", calibration_sum);

    let calibration_values_with_words: Vec<_> = raw_calibration_values
        .iter()
        .map(|r| {
            let digits: &[_] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
            let words: &[_] = &[
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];

            let digit_matches: Vec<_> = digits
                .iter()
                .chain(words.iter())
                .filter_map(|digit| r.find(digit).map(|idx| (idx, digit)))
                .map(|(idx, digit)| {
                    let integer = digits
                        .iter()
                        .position(|d| d == digit)
                        .or(words.iter().position(|d| d == digit))
                        .unwrap()
                        + 1; // array index to number
                    (idx, integer)
                })
                .collect();

            println!("{:?} {:?}", r, digit_matches);

            let first_digit = digit_matches.iter().min_by_key(|&(idx, _)| idx).unwrap().1;
            let last_digit = digit_matches.iter().max_by_key(|&(idx, _)| idx).unwrap().1;
            first_digit * 10 + last_digit
        })
        .collect();

    let calibration_sum_with_words: usize = calibration_values_with_words.iter().sum();
    println!("Calibration value sum: {}", calibration_sum_with_words);
    Ok(())
}
