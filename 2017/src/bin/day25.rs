extern crate regex;

use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let mut input_buf = File::open(&env::args().nth(1).unwrap()).unwrap();
    let mut input = String::new();
    let _ = input_buf.read_to_string(&mut input).unwrap();

    let start_re = Regex::new(r"Begin in state ([A-Z]).").unwrap();
    let checksum_re = Regex::new(r"Perform a diagnostic checksum after ([0-9]+) steps.").unwrap();

    let state_re = Regex::new(
        r"In state ([A-Z]):
  If the current value is 0:
    - Write the value ([01]).
    - Move one slot to the (\w+).
    - Continue with state ([A-Z]).
  If the current value is 1:
    - Write the value ([01]).
    - Move one slot to the (\w+).
    - Continue with state ([A-Z]).",
    ).unwrap();

    let mut state = start_re
        .captures(&input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .chars()
        .nth(0)
        .unwrap();

    let checksum_length: usize = checksum_re
        .captures(&input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    let mut state_mach = HashMap::new();
    for cap in state_re.captures_iter(&input) {
        let key = cap.get(1).unwrap().as_str().chars().nth(0).unwrap();
        let val0: usize = cap.get(2).unwrap().as_str().parse().unwrap();
        let dir0 = cap.get(3).unwrap().as_str();
        let next0 = cap.get(4).unwrap().as_str().chars().nth(0).unwrap();
        let val1: usize = cap.get(5).unwrap().as_str().parse().unwrap();
        let dir1 = cap.get(6).unwrap().as_str();
        let next1 = cap.get(7).unwrap().as_str().chars().nth(0).unwrap();
        state_mach.insert(key, [(val0, dir0, next0), (val1, dir1, next1)]);
    }

    let mut tape = HashMap::new();
    let mut pos = 0;
    for _ in 0..checksum_length {
        let prog = state_mach[&state];
        let val = *tape.entry(pos).or_insert(0);
        *tape.entry(pos).or_insert(0) = prog[val].0;
        pos += if prog[val].1 == "right" { 1 } else { -1 };
        state = prog[val].2;
    }
    println!("Checksum: {}", tape.values().filter(|&&v| v == 1).count());
}
