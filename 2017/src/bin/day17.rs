use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input: usize = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();

    let mut buf = Vec::new();
    buf.insert(0, 0);
    let mut cur_pos = 0;
    for i in 1..2017 + 1 {
        let pos = (cur_pos + input) % buf.len();
        buf.insert(pos + 1, i);
        cur_pos = pos + 1;
    }
    println!("Number after 2017: {}", buf[cur_pos + 1]);

    cur_pos = 0;
    let mut last_idx = 0;
    for i in 1..50_000_000 + 1 {
        let pos = (cur_pos + input) % i;
        if pos == 0 {
            last_idx = i;
        }
        cur_pos = pos + 1;
    }
    println!("Number after 0: {}", last_idx);
}
