use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

extern crate advent_of_code;
use advent_of_code::intcode;

fn main() -> io::Result<()> {
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

    let mut mach = intcode::Machine::new(&program, &[1]);
    let output = loop {
        let res = mach.run_until_output();
        if res != Some(0) {
            break res;
        }
    };
    println!("TEST diagnostic code: {}", output.unwrap());

    let mut mach = intcode::Machine::new(&program, &[5]);
    let output = mach.run_until_output();
    println!("Radiator diagnostic code: {}", output.unwrap());

    Ok(())
}
