use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// mm, massive oneliners
fn scramble(operations: &[String], input: String) -> String {
    return operations.iter().fold(input, |input, operation| {
        let mut chars: Vec<_> = input.chars().collect();
        if operation.starts_with("swap") {
            let (ix, jx) = if operation.starts_with("swap position") {
                (
                    operation.chars().nth(14).unwrap().to_digit(10).unwrap() as usize,
                    operation.chars().last().unwrap().to_digit(10).unwrap() as usize,
                )
            } else {
                // swap letter
                let chr_a = operation.chars().nth(12).unwrap();
                let chr_b = operation.chars().last().unwrap();
                (input.find(chr_a).unwrap(), input.find(chr_b).unwrap())
            };

            chars.swap(ix, jx);
        } else if operation.starts_with("rotate") {
            if operation.starts_with("rotate based") {
                let chr = operation.chars().last().unwrap();
                let chr_ix = input.find(chr).unwrap() as usize;
                let rotate_n = (chr_ix + 1 + if chr_ix >= 4 { 1 } else { 0 }) % chars.len();
                chars.rotate_right(rotate_n);
            } else if operation.starts_with("rotate left") {
                let rotate_n = operation.chars().nth(12).unwrap().to_digit(10).unwrap();
                chars.rotate_left(rotate_n as usize);
            } else {
                // rotate right
                let rotate_n = operation.chars().nth(13).unwrap().to_digit(10).unwrap();
                chars.rotate_right(rotate_n as usize);
            }
        } else if operation.starts_with("reverse") {
            let ix = operation.chars().nth(18).unwrap().to_digit(10).unwrap() as usize;
            let jx = operation.chars().last().unwrap().to_digit(10).unwrap() as usize;
            chars = [
                &chars[..ix],
                chars[usize::min(ix, jx)..=usize::max(ix, jx)]
                    .into_iter()
                    .copied()
                    .rev()
                    .collect::<Vec<char>>()
                    .as_slice(),
                &chars[jx + 1..],
            ]
            .concat();
        } else if operation.starts_with("move") {
            let ix = operation.chars().nth(14).unwrap().to_digit(10).unwrap() as usize;
            let jx = operation.chars().last().unwrap().to_digit(10).unwrap() as usize;
            let chr = chars.remove(ix);
            chars.insert(jx, chr);
        } else {
            panic!("Unrecognised operation: {}", operation);
        }
        return chars.into_iter().collect();
    });
}

fn unscramble(operations: &[String], input: String) -> String {
    return operations.iter().rev().fold(input, |input, operation| {
        let mut chars: Vec<_> = input.chars().collect();
        if operation.starts_with("swap") {
            let (ix, jx) = if operation.starts_with("swap position") {
                (
                    operation.chars().nth(14).unwrap().to_digit(10).unwrap() as usize,
                    operation.chars().last().unwrap().to_digit(10).unwrap() as usize,
                )
            } else {
                // swap letter
                let chr_a = operation.chars().nth(12).unwrap();
                let chr_b = operation.chars().last().unwrap();
                (input.find(chr_a).unwrap(), input.find(chr_b).unwrap())
            };

            chars.swap(ix, jx);
        } else if operation.starts_with("rotate") {
            if operation.starts_with("rotate based") {
                // TODO
                let chr = operation.chars().last().unwrap();
                let chr_ix = input.find(chr).unwrap() as usize;
                // XXX big ol' hack, assumes string length of 8.
                // There's definitely a better way to do this, but I can't figure it out
                // mapping:
                // before -> scrambled r<right rotations> l<left rotations>
                // 0 -> 1 r1 l1
                // 1 -> 3 r2 l2
                // 2 -> 5 r3 l3
                // 3 -> 7 r4 l4
                // 4 -> 2 r6 l6
                // 5 -> 4 r7 l7
                // 6 -> 6 r8 l8
                // 7 -> 0 r9 l9
                let rotate_n = match chr_ix {
                    0 => 9,
                    1 => 1,
                    2 => 6,
                    3 => 2,
                    4 => 7,
                    5 => 3,
                    6 => 8,
                    7 => 4,
                    _ => unreachable!(),
                } % chars.len();
                chars.rotate_left(rotate_n);
            } else if operation.starts_with("rotate left") {
                let rotate_n = operation.chars().nth(12).unwrap().to_digit(10).unwrap();
                chars.rotate_right(rotate_n as usize);
            } else {
                // rotate right
                let rotate_n = operation.chars().nth(13).unwrap().to_digit(10).unwrap();
                chars.rotate_left(rotate_n as usize);
            }
        } else if operation.starts_with("reverse") {
            let ix = operation.chars().nth(18).unwrap().to_digit(10).unwrap() as usize;
            let jx = operation.chars().last().unwrap().to_digit(10).unwrap() as usize;
            chars = [
                &chars[..ix],
                chars[usize::min(ix, jx)..=usize::max(ix, jx)]
                    .into_iter()
                    .copied()
                    .rev()
                    .collect::<Vec<char>>()
                    .as_slice(),
                &chars[jx + 1..],
            ]
            .concat();
        } else if operation.starts_with("move") {
            let ix = operation.chars().last().unwrap().to_digit(10).unwrap() as usize;
            let jx = operation.chars().nth(14).unwrap().to_digit(10).unwrap() as usize;
            let chr = chars.remove(ix);
            chars.insert(jx, chr);
        } else {
            panic!("Unrecognised operation: {}", operation);
        }
        return chars.into_iter().collect();
    });
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    // One liner heyo
    let operations = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();

    // reversing test loop
    for n in 0..operations.len() {
        let ops = &operations[0..n];
        let scr = scramble(ops, "abcdefgh".to_string());
        let unscr = unscramble(ops, scr.clone());
        if !unscr.eq("abcdefgh") {
            println!("abcdefgh -> {} -> {}\n{:?}", scr, unscr, ops);
            break;
        }
    }

    println!(
        "Scrambled password '{}': {}",
        "abcdefgh",
        scramble(&operations, "abcdefgh".to_string())
    );

    // fbgdceah
    println!(
        "Unscrambled password '{}': {}",
        "fbgdceah",
        unscramble(&operations, "fbgdceah".to_string())
    );
}
