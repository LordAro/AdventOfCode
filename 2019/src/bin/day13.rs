use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use advent_of_code::intcode;

fn main() {
    let program_str = BufReader::new(
        File::open(
            env::args()
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

    let mut p1 = false;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut starting_blocks = 0;
    let mut score = 0;
    let mut mach = intcode::Machine::new(&program, &[]);
    mach.set_memory(0, 2); // Play
    loop {
        match mach.run() {
            intcode::RunRetVal::Halted => break,
            intcode::RunRetVal::NeedsInput => {
                p1 = true;
                mach.push_input(ball_x.cmp(&paddle_x) as isize);
            }
            intcode::RunRetVal::Output(x) => {
                let y = mach.run_until_output().unwrap();
                let tileid = mach.run_until_output().unwrap();
                match tileid {
                    2 => {
                        if !p1 {
                            starting_blocks += 1
                        }
                    }
                    3 => paddle_x = x,
                    4 => ball_x = x,
                    _ => {}
                }
                if (x, y) == (-1, 0) {
                    score = tileid;
                }
            }
        }
    }

    println!("Number of blocks: {}", starting_blocks);
    println!("Score: {}", score);
}
