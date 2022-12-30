extern crate itertools;

use itertools::iterate;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let mut input: Vec<u64> = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    let mut input2 = input.clone();

    let mut score = 0;
    for _ in 0..40_000_000 {
        input[0] = (input[0] * 16807) % 0x7fffffff;
        input[1] = (input[1] * 48271) % 0x7fffffff;
        if input[0] & 0xffff == input[1] & 0xffff {
            score += 1;
        }
    }
    println!("Score: {}", score);

    let mut score2 = 0;
    for _ in 0..5_000_000 {
        input2[0] = iterate(input2[0], |&i| (i * 16807) % 0x7fffffff)
            .skip(1)
            .find(|i| i % 4 == 0)
            .unwrap();
        input2[1] = iterate(input2[1], |&i| (i * 48271) % 0x7fffffff)
            .skip(1)
            .find(|i| i % 8 == 0)
            .unwrap();
        if input2[0] & 0xffff == input2[1] & 0xffff {
            score2 += 1;
        }
    }
    println!("Score2: {}", score2);
}
