use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let mut input: Vec<i32> = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let mut input2 = input.clone();

    let mut n: i32 = 0;
    let mut steps = 0;
    while n >= 0 && n < input.len() as i32 {
        let jump = input[n as usize];
        input[n as usize] += 1;
        n += jump;
        steps += 1;
    }
    println!("Steps: {}", steps);

    n = 0;
    steps = 0;
    while n >= 0 && n < input2.len() as i32 {
        let jump = input2[n as usize];
        input2[n as usize] += if jump >= 3 { -1 } else { 1 };
        n += jump;
        steps += 1;
    }
    println!("Steps 2: {}", steps);
}
