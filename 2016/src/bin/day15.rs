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
    (r.1, s.1, t.1)
}

// https://crypto.stanford.edu/pbc/notes/numbertheory/crt.html
// for series of equations x = a_i mod m_i
// x = sum(a_i * b_i * b'_i)
// where b = M / m_i
// where M = product(m_1 .. m_n)
// where b' = multiplicative inverse of b mod m
fn get_release_time(discs: &[(i64, i64)]) -> i64 {
    let big_m: i64 = discs.iter().map(|(_, m)| m).product();

    discs
        .iter()
        .map(|&(a, m)| {
            let b = big_m / m;
            let (_r, s, _) = extended_gcd(b, m);
            // r (gcd) always equals 1 ...hopefully
            let b_inverse = s;
            a * b * b_inverse
        })
        .sum::<i64>()
        .rem_euclid(big_m)
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap());

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
                caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            )
        })
        .enumerate()
        // 1-indexed, and make sure it's positive
        .map(|(i, (num, pos))| ((num - pos - ((i as i64) + 1)).rem_euclid(num), num))
        .collect::<_>();

    let time1 = get_release_time(&discs);

    println!("Button press time: {}", time1);

    // new disc, 11 positions, starting at position 0, below bottom disc
    let new_disc = vec![((11 - (discs.len() as i64 + 1)).rem_euclid(11), 11)];
    let discs2 = discs.into_iter().chain(new_disc).collect::<Vec<_>>();
    let time2 = get_release_time(&discs2);
    println!("Button press time with additional disc: {}", time2);
}
