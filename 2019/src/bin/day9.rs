use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate advent_of_code;
use advent_of_code::intcode;

fn main() {
    let program_str = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .next()
    .unwrap()
    .unwrap();

    let program = intcode::read_input(&program_str);

    let mut mach = intcode::Machine::new(&program, &[1]); // test mode
    while let Some(output) = mach.run() {
        println!("BOOST keycode: {}", output);
    }
    let mut mach = intcode::Machine::new(&program, &[2]); // sensor boost mode
    while let Some(output) = mach.run() {
        println!("Distress signal coordinates: {}", output);
    }
}
