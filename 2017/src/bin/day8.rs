use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let input: Vec<Vec<_>> = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
        .collect();

    let mut map = HashMap::new();
    let mut abs_largest = 0;
    input.iter().for_each(|v| {
        // Array destructuring is experimental :(
        let reg = v.get(0).unwrap();
        let change_dir = if v.get(1).unwrap() == "inc" { 1 } else { -1 };
        let chg_amt: i32 = v.get(2).unwrap().parse().unwrap();
        // v.get(3) == "if"
        let cond_reg = v.get(4).unwrap();
        let cond_op = v.get(5).unwrap();
        let cond_r: i32 = v.get(6).unwrap().parse().unwrap();

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
