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

    let max_layer = input.last().unwrap().0;
    let mut layers = vec![0; max_layer + 1];
    for (i, v) in input {
        layers[i] = v;
    }

    let penalty = layers
        .iter()
        .enumerate()
        .filter(|&(i, &d)| d != 0 && i % ((2 * d) - 2) == 0)
        .fold(0, |a, (i, &d)| a + i * d);
    println!("Penalty with 0 delay: {}", penalty);

    for delay in 0.. {
        if layers
            .iter()
            .enumerate()
            .filter(|&(i, &d)| d != 0 && (i + delay) % ((2 * d) - 2) == 0)
            .count() == 0
        {
            println!("Delay required for 0 penalty: {}", delay);
            break;
        }
    }
}
