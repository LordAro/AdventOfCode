use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let mut score = 0;
    let mut open_groups = 0;
    let mut in_garbage = false;
    let mut garbage_count = 0;
    let mut skip_next = false;
    for c in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if !in_garbage {
            match c {
                '{' => open_groups += 1,
                '}' => {
                    score += open_groups;
                    open_groups -= 1
                }
                '<' => in_garbage = true,
                _ => {}
            }
        } else {
            match c {
                '!' => skip_next = true,
                '>' => in_garbage = false,
                _ => garbage_count += 1,
            }
        }
    }
    println!("Score: {}", score);
    println!("Garbage count: {}", garbage_count);
}
