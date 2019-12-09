use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

extern crate rayon;
use rayon::prelude::*;

#[macro_use]
extern crate itertools;

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

    let mut program = intcode::read_input(&program_str);
    program[1] = 12;
    program[2] = 2;

    let mut mach = intcode::Machine::new(&program, &[]);
    mach.run();
    println!("Output(12, 2): {}", mach.get_memory(0));

    let (n, v) = iproduct!(0..100, 0..100)
        .par_bridge()
        .find_any(|(n, v)| {
            let mut new_prog = program.clone();
            new_prog[1] = *n;
            new_prog[2] = *v;
            let mut mach = intcode::Machine::new(&new_prog, &[]);
            mach.run();
            let result = mach.get_memory(0);
            result == 19690720
        })
        .unwrap();
    println!("Output({}, {}): 19690720, or: {}", n, v, 100 * n + v);

    Ok(())
}
