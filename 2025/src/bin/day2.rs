use itertools::Itertools;
use std::env;
use std::fs;
use std::io;
use std::iter;

fn main() -> io::Result<()> {
    let input: Vec<_> = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
        .trim()
        .split(',')
        .map(|raw_range| {
            let (s, e) = raw_range.split_once('-').unwrap();
            (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap())
        })
        .collect();

    let mut p1_invalid_sum = 0;
    let mut p2_invalid_sum = 0;
    for (start, end) in input {
        for n in start..=end {
            let num_digits = n.ilog10() + 1;
            for i in 1..=num_digits / 2 {
                if num_digits % i != 0 {
                    continue;
                }
                let half = 10_i32.pow(i) as usize;
                if iter::successors(Some(n), |n2| n2.checked_div(half))
                    .take_while(|n2| *n2 > 0)
                    .map(|n2| n2 % half)
                    .all_equal()
                {
                    if i == num_digits / 2 && num_digits % 2 == 0 {
                        p1_invalid_sum += n;
                    }
                    p2_invalid_sum += n;
                    break; // can't be an invalid id more than once
                }
            }
        }
    }

    println!("P1: Invalid ID sum: {p1_invalid_sum}");
    println!("P2: Invalid ID sum (new rules): {p2_invalid_sum}");
    Ok(())
}
