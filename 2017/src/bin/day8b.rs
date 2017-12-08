extern crate time;

use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use time::Duration;

fn read_input(input_file: &str) -> Vec<Vec<String>> {
    BufReader::new(File::open(input_file).unwrap())
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
        .collect()
}

fn process(input: Vec<Vec<String>>) -> (i32, i32) {
    let mut map = HashMap::new();
    let mut abs_largest = 0;
    input.iter().for_each(|v| {
        // Array destructuring is experimental :(
        let reg = v.get(0).unwrap();
        let change_dir = if v.get(1).unwrap() == "inc" { 1 } else { -1 };
        let chg_amt: i32 = v.get(2).unwrap().parse().unwrap();
        // v.get(3) == "if"
        let cond_reg = v.get(4).unwrap();
        let cond_op = v.get(5).unwrap();
        let cond_r: i32 = v.get(6).unwrap().parse().unwrap();

        let cond_val = *map.entry(cond_reg).or_insert(0);
        let cond = cond_op == "==" && cond_val == cond_r || cond_op == "!=" && cond_val != cond_r ||
            cond_op == ">" && cond_val > cond_r ||
            cond_op == "<" && cond_val < cond_r ||
            cond_op == ">=" && cond_val >= cond_r ||
            cond_op == "<=" && cond_val <= cond_r;
        if cond {
            *map.entry(reg).or_insert(0) += chg_amt * change_dir;
            abs_largest = cmp::max(*map.get(reg).unwrap(), abs_largest);
        }
    });

    let largest = map.iter().max_by(|&(_, v1), &(_, v2)| v1.cmp(v2)).unwrap();
    return (*largest.1, abs_largest);
}

fn main() {
    let mut total1 = Duration::zero();
    let mut total2 = Duration::zero();
    for _ in 0..10000 {
        let now1 = time::now();
        let input = read_input("inputs/day8.input"); // don't duplicate
        let now2 = time::now();

        let (largest, abs_largest) = process(input);
        let now3 = time::now();
        println!("Largest value: {:?}", largest);
        println!("Absolute largest value: {:?}", abs_largest);
        total1 = total1 + (now2 - now1);
        total2 = total2 + (now3 - now2);
    }
    println!(
        "Input reading: {:?}us",
        (total1 / 10000).num_microseconds().unwrap()
    );
    println!(
        "Processing: {:?}us",
        (total2 / 10000).num_microseconds().unwrap()
    );
}
