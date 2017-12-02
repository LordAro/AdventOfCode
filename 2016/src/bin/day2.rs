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

fn up(p: i32) -> Option<i32> {
    match p {
        1 | 2 | 3 => Some(p),
        4 | 5 | 6 => Some(p - 3),
        7 | 8 | 9 => Some(p - 3),
        _ => None,
    }
}
fn down(p: i32) -> Option<i32> {
    match p {
        1 | 4 | 7 => Some(p + 1),
        2 | 5 | 8 => Some(p + 1),
        3 | 6 | 9 => Some(p),
        _ => None,
    }
}
fn left(p: i32) -> Option<i32> {
    match p {
        1 | 4 | 7 => Some(p),
        2 | 5 | 8 => Some(p - 1),
        3 | 6 | 9 => Some(p - 1),
        _ => None,
    }
}

fn right(p: i32) -> Option<i32> {
    match p {
        1 | 2 | 3 => Some(p + 3),
        4 | 5 | 6 => Some(p + 3),
        7 | 8 | 9 => Some(p),
        _ => None,
    }
}


fn up2(p: i32) -> Option<i32> {
    match p {
        1 | 2 | 4 | 5 | 9 => Some(p),
        6 | 7 | 8 | 0xA | 0xB | 0xC => Some(p - 4),
        3 | 0xD => Some(p - 2),
        _ => None,
    }
}
fn down2(p: i32) -> Option<i32> {
    match p {
        5 | 9 | 0xA | 0xC | 0xD => Some(p),
        1 | 0xB => Some(p + 2),
        2 | 3 | 4 | 6 | 7 | 8 => Some(p + 4),
        _ => None,
    }
}
fn left2(p: i32) -> Option<i32> {
    match p {
        1 | 2 | 5 | 0xA | 0xD => Some(p),
        3 | 4 | 6 | 7 | 8 | 9 | 0xB | 0xC => Some(p - 1),
        _ => None,
    }
}
fn right2(p: i32) -> Option<i32> {
    match p {
        1 | 4 | 9 | 0xC | 0xD => Some(p),
        2 | 3 | 5 | 6 | 7 | 8 | 0xA | 0xB => Some(p + 1),
        _ => None,
    }
}

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
    let mut code2 = String::new();
    let mut cur_pos = 5; // Starting point
    let mut cur_pos2 = 5;

    for line in file_buf.lines() {
        let l = line.unwrap();
        for c in l.chars() {
            cur_pos2 = match c {
                'U' => up2(cur_pos2),
                'L' => left2(cur_pos2),
                'R' => right2(cur_pos2),
                'D' => down2(cur_pos2),
                _ => panic!("Unrecognised character"),
            }.unwrap();
            cur_pos = match c {
                'U' => up(cur_pos),
                'L' => left(cur_pos),
                'R' => right(cur_pos),
                'D' => down(cur_pos),
                _ => panic!("Unrecognised character"),
            }.unwrap();
        }
        code.push_str(&cur_pos.to_string());
        code2.push_str(&format!("{:X}", cur_pos2));
    }
    println!("Bathroom code: {}", code);
    println!("Actual bathroom code: {}", code2);
}
