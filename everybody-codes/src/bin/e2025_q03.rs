use itertools::Itertools;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_total: usize = fs::read_to_string(p1_input_filename)?
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .sorted()
        .dedup()
        .sum();

    let p2_total: usize = fs::read_to_string(p2_input_filename)?
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .sorted()
        .dedup()
        .take(20)
        .sum();

    let mut p3_input: Vec<_> = fs::read_to_string(p3_input_filename)?
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .sorted()
        .collect();

    let mut num_sets = 0;
    while !p3_input.is_empty() {
        let mut prev = 0;
        // don't actually need the result
        p3_input.retain(|x| {
            if *x <= prev {
                return true;
            }
            prev = *x;
            false
        });
        num_sets += 1;
    }

    println!("P1: Maximum packing size: {p1_total}");
    println!("P2: Minimum packing size of 20: {p2_total}");
    println!("P3: Minimum number of sets: {num_sets}");
    Ok(())
}
