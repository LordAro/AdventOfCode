use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

// P1:
// 1 2 3
// 4 5 6
// 7 8 9
//
// P2:
//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let f = match File::open(&env::args().nth(1).unwrap()) {
        Err(why) => panic!("Could not open input file: {}\n", why),
        Ok(fh) => fh,
    };
    let file_buf = BufReader::new(&f);

    let mut code = String::new();
    let mut cur_pos = 5; // Starting point
    for line in file_buf.lines() {
        let l = line.unwrap();
        for c in l.chars() {
            cur_pos = match c {
                'U' => {
                    match cur_pos {
                        1 | 2 | 3 => cur_pos,
                        4 | 5 | 6 => cur_pos - 3,
                        7 | 8 | 9 => cur_pos - 3,
                        _ => panic!("Weird position: {}", cur_pos),
                    }
                }
                'L' => {
                    match cur_pos {
                        1 | 4 | 7 => cur_pos,
                        2 | 5 | 8 => cur_pos - 1,
                        3 | 6 | 9 => cur_pos - 1,
                        _ => panic!("Weird position: {}", cur_pos),
                    }
                }
                'R'_ => {
                    match cur_pos {
                        1 | 4 | 7 => cur_pos + 1,
                        2 | 5 | 8 => cur_pos + 1,
                        3 | 6 | 9 => cur_pos,
                        _ => panic!("Weird position: {}", cur_pos),
                    }
                }
                'D' => {
                    match cur_pos {
                        1 | 2 | 3 => cur_pos + 3,
                        4 | 5 | 6 => cur_pos + 3,
                        7 | 8 | 9 => cur_pos,
                        _ => panic!("Weird position: {}", cur_pos),
                    }
                }
                _ => panic!("Unrecognised character"),
            };
        }
        code.push_str(&cur_pos.to_string());
    }
    println!("Bathroom code: {}", code);
}
