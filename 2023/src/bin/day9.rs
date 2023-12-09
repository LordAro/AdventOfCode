use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn extrapolate_next(list: &[i64]) -> i64 {
    let mut diffs = list.to_vec();
    let mut last_val_sum = 0;
    while diffs.iter().any(|&n| n != 0) {
        diffs = diffs
            .windows(2)
            .map(|arr| match arr {
                &[a, b] => b - a,
                _ => unreachable!(),
            })
            .collect();
        last_val_sum += diffs.last().unwrap();
    }
    list.last().unwrap() + last_val_sum
}

fn main() -> io::Result<()> {
    let input_data: Vec<Vec<i64>> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| {
        l.unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    })
    .collect();

    let extrapolated_sum: i64 = input_data.iter().map(|list| extrapolate_next(list)).sum();
    println!("Sum of extrapolated values: {}", extrapolated_sum);

    // Just reverse the lists
    let rev_extrapolated_sum: i64 = input_data
        .iter()
        .map(|list| extrapolate_next(&list.iter().rev().copied().collect::<Vec<_>>()))
        .sum();

    println!(
        "Sum of extrapolated reversed values: {}",
        rev_extrapolated_sum
    );

    Ok(())
}
