use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input_str: String = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let input: Vec<_> = input_str.split(',').map(|i| i.parse().unwrap()).collect();
    let mut rope: Vec<usize> = (0..256).collect();

    let mut pos = 0;
    for (skip, &ins) in input.iter().enumerate() {
        let subrope: Vec<_> = rope.iter().cycle().skip(pos).take(ins).cloned().collect();
        let subrope_r: Vec<_> = subrope.iter().rev().collect();
        let len = rope.len();
        for (i, &&r) in subrope_r.iter().enumerate() {
            rope[(pos + i) % len] = r;
        }
        pos = (pos + ins + skip) % 256;
    }
    println!("Multiplication: {}", rope[0] * rope[1]);

    let num_rounds = 64;
    let additional = vec![17, 31, 73, 47, 23];

    let input2 = [input_str.as_bytes(), &additional[..]].concat();
    rope = (0..256).collect(); // reset
    pos = 0;

    for r in 0..num_rounds {
        for (skip, &ins) in input2.iter().enumerate() {
            let subrope: Vec<_> = rope
                .iter()
                .cycle()
                .skip(pos)
                .take(ins as usize)
                .cloned()
                .collect();
            let subrope_r: Vec<_> = subrope.iter().rev().collect();
            let len = rope.len();
            for (i, &&r) in subrope_r.iter().enumerate() {
                rope[(pos + i) % len] = r;
            }
            pos = (pos + ins as usize + skip + (r * input2.len())) % 256;
        }
    }
    let hash: String = rope
        .chunks(16)
        .map(|c| c.iter().fold(0, |a, b| a ^ b))
        .map(|e| format!("{:02x}", e))
        .collect();
    println!("Dense hash: {}", hash);
}
