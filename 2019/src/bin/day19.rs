use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use advent_of_code::intcode;
use itertools::Itertools;

fn is_point_covered(program: &[isize], x: isize, y: isize) -> bool {
    intcode::Machine::new(program, &[x, y])
        .run_until_output()
        .filter(|&p| p == 1)
        .is_some()
}

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

    let points_affected = (0..50)
        .cartesian_product(0..50)
        .filter(|&(x, y)| is_point_covered(&program, x, y))
        .count();
    println!(
        "Number of points affected by tractor beam: {}",
        points_affected
    );

    // cheap and nasty exit
    let mut prev_x = 0;
    'outer: for y in 0.. {
        // first position must be(?) within 2n of the first position found
        let mut next_prev_x = prev_x;
        for x in prev_x..prev_x + 200 {
            if !is_point_covered(&program, x, y) {
                continue;
            }
            next_prev_x = x;

            if !is_point_covered(&program, x + 99, y) {
                // can't be this row, move on to the next
                break;
            }

            if !is_point_covered(&program, x, y + 99) {
                // might be further along the x-axis, keep checking
                continue;
            }

            // don't need to check the 4th corner at all

            println!(
                "Found corners of 100x100 square: {x} {y}, {}",
                x * 10000 + y
            );
            break 'outer;
        }
        prev_x = next_prev_x;
    }
}
