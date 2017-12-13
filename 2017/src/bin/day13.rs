use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let input: Vec<(usize, usize)> = BufReader::new(
        File::open(&env::args().nth(1).unwrap()).unwrap(),
    ).lines()
        .map(|l| {
            let v: Vec<_> = l.unwrap().split(": ").map(|e| e.parse().unwrap()).collect();
            (v[0], v[1])
        })
        .collect();

    let penalty = input
        .iter()
        .filter(|&&(i, d)| i % ((2 * d) - 2) == 0)
        .fold(0, |a, &(i, d)| a + i * d);
    println!("Penalty with 0 delay: {}", penalty);

    let delay = (0..)
        .find(|delay| {
            input
                .iter()
                .filter(|&&(i, d)| (i + delay) % ((2 * d) - 2) == 0)
                .count() == 0
        })
        .unwrap();
    println!("Delay required for 0 penalty: {}", delay);
}
