extern crate itertools;

use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input: Vec<Vec<_>> = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| {
            l.unwrap().split_whitespace().map(String::from).collect()
        })
        .collect();

    let valid1: usize = input
        .iter()
        .map(|v| v.iter().sorted())
        .filter(|v| v.into_iter().dedup().count() == v.len())
        .count();

    let valid2: usize = input
        .iter()
        .map(|v| v.into_iter().map(|w| w.chars().sorted()).sorted())
        .filter(|v| v.into_iter().dedup().count() == v.len())
        .count();

    println!("Valid passphrases: {:?}", valid1);
    println!("New valid passphrases: {:?}", valid2);
}
