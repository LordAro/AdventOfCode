use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};

fn score(input: &Vec<(usize, usize)>) -> usize {
    input.iter().fold(0, |acc, &(a, b)| acc + a + b)
}

fn find_longest(input: &Vec<(usize, usize)>, n: usize) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .filter(|&(_, &(a, b))| a == n || b == n)
        .map(|(i, &p)| {
            let mut input_cl = input.clone();
            input_cl.swap_remove(i);
            let other = if p.0 == n { p.1 } else { p.0 };
            let mut v = find_longest(&input_cl, other);
            v.push(p);
            v
        })
        .max_by(|a, b| a.len().cmp(&b.len()).then(score(a).cmp(&score(b))))
        .unwrap_or(Vec::new())
}

fn find_strongest(input: &Vec<(usize, usize)>, n: usize) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .filter(|&(_, &(a, b))| a == n || b == n)
        .map(|(i, &p)| {
            let mut input_cl = input.clone();
            input_cl.swap_remove(i);
            let other = if p.0 == n { p.1 } else { p.0 };
            let mut v = find_strongest(&input_cl, other);
            v.push(p);
            v
        })
        .max_by_key(|v| score(v))
        .unwrap_or(Vec::new())
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input: Vec<(usize, usize)> = BufReader::new(
        File::open(&env::args().nth(1).unwrap()).unwrap(),
    ).lines()
        .map(|l| {
            let v: Vec<_> = l.unwrap().split('/').map(|n| n.parse().unwrap()).collect();
            (v[0], v[1])
        })
        .collect();

    println!("Strongest bridge: {}", score(&find_strongest(&input, 0)));
    println!("Longest bridge: {}", score(&find_longest(&input, 0)));
}
