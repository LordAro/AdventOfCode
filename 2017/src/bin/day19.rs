use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input: Vec<Vec<char>> = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut foundstr = String::new();
    let mut steps = 0;

    // Find start (y, x)
    let mut pos = (0, input[0].iter().position(|&c| c == '|').unwrap());
    let mut dir = (1, 0);
    // Follow line until gone off the end
    while input[pos.0][pos.1] != ' ' {
        // Follow line until find a direction change
        while input[pos.0][pos.1] != '+' && input[pos.0][pos.1] != ' ' {
            if input[pos.0][pos.1].is_alphabetic() {
                foundstr.push(input[pos.0][pos.1]);
            }
            pos = (
                (pos.0 as i32 + dir.0) as usize,
                (pos.1 as i32 + dir.1) as usize,
            );
            steps += 1;
        }
        // Determine new direction
        let newdirs = if dir.0.abs() == 1 {
            [(0, 1), (0, -1)]
        } else {
            [(1, 0), (-1, 0)]
        };
        for newdir in &newdirs {
            let newpos = (
                (pos.0 as i32 + newdir.0) as usize,
                (pos.1 as i32 + newdir.1) as usize,
            );
            if input[newpos.0][newpos.1] != ' ' {
                pos = newpos;
                dir = *newdir;
                steps += 1;
                break;
            }
        }
    }
    println!("Route string: {}", foundstr);
    println!("Total steps: {}", steps);
}
