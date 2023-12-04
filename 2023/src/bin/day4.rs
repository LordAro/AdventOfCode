use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

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

    let cards: Vec<(HashSet<u32>, HashSet<u32>)> = input_data
        .iter()
        .map(|l| {
            let without_prefix = l.split(':').nth(1).unwrap();
            let mut it = without_prefix.split('|');
            let winning_numbers_str = it.next().unwrap();
            let my_numbers_str = it.next().unwrap();
            let winning_numbers = winning_numbers_str
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let my_numbers = my_numbers_str
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (winning_numbers, my_numbers)
        })
        .collect();

    let matching_card_counts: Vec<usize> = cards
        .iter()
        .map(|(my, win)| my.intersection(win).count())
        .collect();

    let winning_score_sum: u32 = matching_card_counts
        .iter()
        .filter(|&&count| count > 0)
        .map(|&count| 2u32.pow(count as u32 - 1))
        .sum();

    println!("Winning score sum: {:?}", winning_score_sum);

    let mut card_counts = vec![1; cards.len()];
    for idx in 0..card_counts.len() {
        for jdx in idx + 1..idx + matching_card_counts[idx] + 1 {
            card_counts[jdx] += card_counts[idx];
        }
    }

    println!("Total scratch cards: {}", card_counts.iter().sum::<usize>());

    Ok(())
}
