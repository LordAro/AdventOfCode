use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn get_value(program: &[isize], idx: usize, op_num: usize) -> isize {
    let op = program[idx];
    let addr_mode = (op / (10_isize.pow(op_num as u32 + 1))) % 10;
    let param = program[idx + op_num];
    if addr_mode == 0 {
        // Position
        program[param as usize]
    } else {
        // Immediate
        param
    }
}

fn set_value(program: &mut [isize], idx: usize, op_num: usize, val: isize) {
    let op = program[idx];
    let addr_mode = (op / (100_isize.pow(op_num as u32))) % 10;
    assert!(
        addr_mode == 0,
        "Invalid addressing mode when writing, got: {}",
        addr_mode
    );
    let param = program[idx + op_num];
    program[param as usize] = val;
}

fn run_program(input_program: &[isize], input_value: isize) -> isize {
    let mut program = input_program.to_vec();
    let mut idx = 0;
    let mut var = input_value;
    loop {
        let op = program[idx] % 100;
        match op {
            1 => {
                // Add
                let new_val = get_value(&program, idx, 1) + get_value(&program, idx, 2);
                set_value(&mut program, idx, 3, new_val);
                idx += 4;
            }
            2 => {
                // Multiply
                let new_val = get_value(&program, idx, 1) * get_value(&program, idx, 2);
                set_value(&mut program, idx, 3, new_val);
                idx += 4;
            }
            3 => {
                // Input
                set_value(&mut program, idx, 1, var);
                idx += 2;
            }
            4 => {
                // Output
                var = get_value(&program, idx, 1);
                idx += 2;
            }
            5 => {
                // Jump if true
                let op1 = get_value(&program, idx, 1);
                let op2 = get_value(&program, idx, 2);
                if op1 != 0 {
                    idx = op2 as usize;
                } else {
                    idx += 3;
                }
            }
            6 => {
                // Jump if false
                let op1 = get_value(&program, idx, 1);
                let op2 = get_value(&program, idx, 2);
                if op1 == 0 {
                    idx = op2 as usize;
                } else {
                    idx += 3;
                }
            }
            7 => {
                // Less than
                let op1 = get_value(&program, idx, 1);
                let op2 = get_value(&program, idx, 2);
                set_value(&mut program, idx, 3, if op1 < op2 { 1 } else { 0 });
                idx += 4;
            }
            8 => {
                // Eq
                let op1 = get_value(&program, idx, 1);
                let op2 = get_value(&program, idx, 2);
                set_value(&mut program, idx, 3, if op1 == op2 { 1 } else { 0 });
                idx += 4;
            }
            99 => break,
            _ => panic!("Unexpected opcode: {}", op),
        }
    }
    return var;
}

fn main() -> io::Result<()> {
    let program: Vec<_> = BufReader::new(
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
    .unwrap()
    .split(',')
    .map(|n| isize::from_str_radix(n, 10).unwrap())
    .collect();

    let output = run_program(&program, 1);
    println!("TEST diagnostic code: {}", output);

    let output = run_program(&program, 5);
    println!("Radiator diagnostic code: {}", output);

    Ok(())
}
