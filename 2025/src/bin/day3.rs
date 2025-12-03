use std::env;
use std::fs;
use std::io;

fn get_joltage<const N: usize>(bank: &[u32]) -> usize {
    let mut max_joltage: usize = 0;
    let mut last_pos = -1isize;
    for i in 0..N {
        let (pos, n) = bank[((last_pos + 1) as usize)..bank.len() - (N - i - 1)]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, n)| **n)
            .unwrap();
        last_pos += 1 + pos as isize;
        max_joltage = max_joltage * 10 + *n as usize;
    }
    max_joltage
}

fn main() -> io::Result<()> {
    let input: Vec<Vec<_>> = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut p1_joltage_sum = 0;
    let mut p2_joltage_sum = 0;
    for bank in input {
        p1_joltage_sum += get_joltage::<2>(&bank);
        p2_joltage_sum += get_joltage::<12>(&bank);
    }
    println!("P1: Max joltage sum: {p1_joltage_sum}");
    println!("P2: Max joltage sum: {p2_joltage_sum}");
    Ok(())
}
