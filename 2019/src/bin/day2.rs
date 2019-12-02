use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn run_program(input_program: &[usize], noun: usize, verb: usize) -> usize {
    let mut program = input_program.to_vec();
    program[1] = noun;
    program[2] = verb;
    let mut idx = 0;
    loop {
        //println!("idx: {}, {:?}", idx, program);
        let op = program[idx];
        let op1 = program[idx + 1];
        let op2 = program[idx + 2];
        let op3 = program[idx + 3];
        match op {
            1 => program[op3] = program[op1] + program[op2],
            2 => program[op3] = program[op1] * program[op2],
            99 => break,
            _ => panic!("Unexpected number"),
        }
        idx += 4;
    }
    return program[0];
}

fn main() -> io::Result<()> {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let program: Vec<_> = BufReader::new(
        File::open(&env::args().nth(1).unwrap()).expect("Could not open input file"),
    )
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .split(',')
    .map(|n| usize::from_str_radix(n, 10).unwrap())
    .collect();

    // Initial state
    println!("Output(12, 2): {}", run_program(&program, 12, 2));

    'outer: for n in 0..100 {
        for v in 0..100 {
            let result = run_program(&program, n, v);
            if result == 19690720 {
                println!("Output({}, {}): 19690720, or: {}", n, v, 100 * n + v);
                break 'outer;
            }
        }
    }

    Ok(())
}
