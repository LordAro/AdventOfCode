extern crate itertools;

use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let sum1: i32 = input
        .iter()
        .map(|l| {
            l.iter().minmax().into_option().map(|t| t.1 - t.0).unwrap()
        })
        .sum();
    println!("Checksum: {}", sum1);

    let sum2: i32 = input
        .iter()
        .map(|l| {
            l.iter()
                .combinations(2)
                .filter(|v| *v.iter().max().unwrap() % *v.iter().min().unwrap() == 0)
                .flat_map(|v| v) // flatten
                .cloned()
                .collect::<Vec<_>>()
        })
        .map(|v| *v.iter().max().unwrap() / *v.iter().min().unwrap())
        .sum();
    println!("Divisible checksum: {:?}", sum2);
}
