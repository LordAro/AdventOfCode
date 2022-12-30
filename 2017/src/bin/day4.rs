extern crate itertools;

use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let input: Vec<Vec<_>> = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| l.unwrap().split_whitespace().map(String::from).collect())
        .collect();

    let valid1: usize = input
        .iter()
        .map(|v| v.iter().sorted())
        .filter(|v| v.iter().dedup().count() == v.len())
        .count();

    let valid2: usize = input
        .iter()
        .map(|v| v.iter().map(|w| w.chars().sorted()).sorted())
        .filter(|v| v.iter().dedup().count() == v.len())
        .count();

    println!("Valid passphrases: {:?}", valid1);
    println!("New valid passphrases: {:?}", valid2);
}
