use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let input: Vec<_> = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let mut map = HashMap::new();
    let mut abs_largest = 0;
    input.iter().for_each(|line| {
        let v: Vec<_> = line.split_ascii_whitespace().collect();

        let reg = v[0];
        let change_dir = if v[1] == "inc" { 1 } else { -1 };
        let chg_amt: i32 = v[2].parse().unwrap();
        // v[3] == "if"
        let cond_reg = v[4];
        let cond_op = v[5];
        let cond_r: i32 = v[6].parse().unwrap();

        let cond_val = *map.entry(cond_reg).or_insert(0);
        let cond = cond_op == "==" && cond_val == cond_r
            || cond_op == "!=" && cond_val != cond_r
            || cond_op == ">" && cond_val > cond_r
            || cond_op == "<" && cond_val < cond_r
            || cond_op == ">=" && cond_val >= cond_r
            || cond_op == "<=" && cond_val <= cond_r;
        if cond {
            *map.entry(reg).or_insert(0) += chg_amt * change_dir;
            abs_largest = cmp::max(*map.get(reg).unwrap(), abs_largest);
        }
    });

    let largest = map.iter().max_by(|&(_, v1), &(_, v2)| v1.cmp(v2)).unwrap();

    println!("Largest value: {:?}", largest);
    println!("Absolute largest value: {:?}", abs_largest);
}
