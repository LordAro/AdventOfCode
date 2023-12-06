use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn run_race_possibilities(max_time: u64) -> Vec<u64> {
    (0..max_time)
        .map(|held_time| {
            let speed = held_time;
            (max_time - held_time) * speed
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input_data: Vec<String> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| l.unwrap())
    .collect();

    let times: Vec<_> = input_data[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let distances_to_beat: Vec<_> = input_data[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let number_of_win_combinations_product: usize = (0..times.len())
        .map(|idx| {
            run_race_possibilities(times[idx])
                .iter()
                .filter(|&&d| d > distances_to_beat[idx])
                .count()
        })
        .product();

    println!(
        "Product of number of ways to win: {}",
        number_of_win_combinations_product
    );

    let concat_time = times
        .iter()
        .fold(0, |acc, n| acc * (10u64.pow(n.ilog10() + 1)) + n);
    let concat_distance = distances_to_beat
        .iter()
        .fold(0, |acc, n| acc * (10u64.pow(n.ilog10() + 1)) + n);
    let number_of_concat_win_combinations = run_race_possibilities(concat_time)
        .iter()
        .filter(|&&d| d > concat_distance)
        .count();

    println!(
        "Number of win combinations (by concatenation): {}",
        number_of_concat_win_combinations
    );
    Ok(())
}
