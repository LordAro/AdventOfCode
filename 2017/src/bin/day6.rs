use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let mut input: Vec<i32> = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut cache = vec![]; // Vec, so that we have index
    let mut cycle = 0;
    while cache.iter().filter(|&v| *v == input).count() == 0 {
        cache.push(input.clone());
        let (mut idx, mut max): (usize, i32) = input
            .iter()
            .enumerate()
            .rev() // want minimum in case of ties
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|x| (x.0, *x.1))
            .unwrap();
        input[idx] = 0;
        while max > 0 {
            idx = (idx + 1) % input.len();
            input[idx] += 1;
            max -= 1;
        }
        cycle += 1;
    }
    println!("Number of redistribution cycles: {}", cycle);

    let idx = cache
        .iter()
        .enumerate()
        .filter(|&(_, v)| *v == input)
        .map(|(i, _)| i)
        .next()
        .unwrap();
    println!("Loop size: {}", cycle - idx);
}
