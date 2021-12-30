extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Returns a list `result` of size 3 where:
// Referring to the equation ax + by = gcd(a, b)
//     result[0] is gcd(a, b)
//     result[1] is x
//     result[2] is y
// Returns r, s & t such that a*s + b*t = r (r == gcd(a, b))
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut s = (0, 1);
    let mut t = (1, 0);
    let mut r = (b, a);
    while r.0 != 0 {
        let quot = r.1 / r.0;
        r = (r.1 - quot * r.0, r.0);
        s = (s.1 - quot * s.0, s.0);
        t = (t.1 - quot * t.0, t.0);
    }
    return (r.1, s.1, t.1);
}

fn _gcd(a_: i64, b_: i64) -> i64 {
    let mut a = a_;
    let mut b = b_;
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    return a;
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap());

    let disc_re =
        Regex::new(r"Disc #[0-9]+ has ([0-9]+) positions; at time=0, it is at position ([0-9]+).")
            .unwrap();

    // Solving equivalence relations:
    // t + disc_num - initial_position === 0 mod disc_positions
    // or
    // t === disc_positions - initial_position - disc_num mod disc_positions
    let discs: Vec<_> = input
        .lines()
        .map(|li| {
            let l = li.unwrap();
            let caps = disc_re.captures(&l).expect("Line not captured by regex");
            (
                caps.at(1).unwrap().parse::<i64>().unwrap(),
                caps.at(2).unwrap().parse::<i64>().unwrap(),
            )
        })
        .enumerate()
        // 1-indexed, and make sure it's positive
        .map(|(i, (num, pos))| ((num - pos - ((i as i64) + 1)).rem_euclid(num), num))
        .collect::<_>();

    let big_m: i64 = discs.iter().map(|(_, m)| m).product();

    let time = discs
        .iter()
        .map(|&(a, m)| {
            let b = big_m / m;
            let (_r, s, _) = extended_gcd(b, m);
            // r (gcd) always equals 1 ...hopefully
            let b_inverse = s;
            return a * b * b_inverse;
        })
        .sum::<i64>()
        .rem_euclid(big_m);

    println!("Disc release time: {}", time);
}
