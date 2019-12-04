use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn split(n: u32) -> (u32, u32, u32, u32, u32, u32) {
    (
        n / 100_000,
        (n / 10_000) % 10,
        (n / 1_000) % 10,
        (n / 100) % 10,
        (n / 10) % 10,
        n % 10,
    )
}

fn digits_ordered(ds: &(u32, u32, u32, u32, u32, u32)) -> bool {
    ds.0 <= ds.1 && ds.1 <= ds.2 && ds.2 <= ds.3 && ds.3 <= ds.4 && ds.4 <= ds.5
}

fn has_adjacent_digits(ds: &(u32, u32, u32, u32, u32, u32)) -> bool {
    ds.0 == ds.1 || ds.1 == ds.2 || ds.2 == ds.3 || ds.3 == ds.4 || ds.4 == ds.5
}

fn has_max_two_adjacent(ds: &(u32, u32, u32, u32, u32, u32)) -> bool {
    ds.0 == ds.1 && ds.1 != ds.2
        || ds.0 != ds.1 && ds.1 == ds.2 && ds.2 != ds.3
        || ds.1 != ds.2 && ds.2 == ds.3 && ds.3 != ds.4
        || ds.2 != ds.3 && ds.3 == ds.4 && ds.4 != ds.5
        || ds.3 != ds.4 && ds.4 == ds.5
}

fn main() -> io::Result<()> {
    let input_str: Vec<u32> = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .split('-')
    .map(|l| l.parse().unwrap())
    .collect();
    let start = input_str[0];
    let end = input_str[1];

    let pw_matches: Vec<_> = (start..=end)
        .map(split)
        .filter(digits_ordered)
        .filter(has_adjacent_digits)
        .collect();
    let p1_count = pw_matches.len();
    let p2_count = pw_matches.into_iter().filter(has_max_two_adjacent).count();
    println!("Number of possible passwords: {}", p1_count);
    println!("Number of possible passwords (part 2): {}", p2_count);
    Ok(())
}
