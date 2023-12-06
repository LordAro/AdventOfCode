use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn get_winning_race_hold_minmax(race_time: u64, distance_to_beat: u64) -> (u64, u64) {
    // (race_time - hold_time) * hold_time > distance_to_beat
    // =>
    // -1 * ( hold_time * hold_time ) + race_time * hold_time - distance_to_beat > 0
    // quadratic = (-b +/- sqrt(b*b - 4ac)) / 2a
    let a: f64 = -1.;
    let b: f64 = race_time as f64;
    let c: f64 = -(distance_to_beat as f64);
    let min = (-b + (b * b - 4. * a * c).sqrt()) / 2. * a;
    let max = (-b - (b * b - 4. * a * c).sqrt()) / 2. * a;
    // not entirely sure why floor & ceil results in a solution that's +1 to the actual result, but
    // there we go
    (min.floor() as u64, max.ceil() as u64)
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

    let number_of_win_combinations_product: u64 = (0..times.len())
        .map(|idx| {
            let minmax = get_winning_race_hold_minmax(times[idx], distances_to_beat[idx]);
            minmax.0.abs_diff(minmax.1) - 1
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

    let concat_minmax = get_winning_race_hold_minmax(concat_time, concat_distance);
    let number_of_concat_win_combinations = concat_minmax.0.abs_diff(concat_minmax.1) - 1;
    println!(
        "Number of win combinations (by concatenation): {}",
        number_of_concat_win_combinations
    );
    Ok(())
}
