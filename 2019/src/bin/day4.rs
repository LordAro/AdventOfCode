use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn is_valid_pw(pw: &u32) -> bool {
    let ds = (
        pw / 100000,
        (pw / 10000) % 10,
        (pw / 1000) % 10,
        (pw / 100) % 10,
        (pw / 10) % 10,
        pw % 10,
    );
    // Shortcircuiting
    if !(ds.0 == ds.1 || ds.1 == ds.2 || ds.2 == ds.3 || ds.3 == ds.4 || ds.4 == ds.5) {
        return false;
    }
    if ds.0 > ds.1 || ds.1 > ds.2 || ds.2 > ds.3 || ds.3 > ds.4 || ds.4 > ds.5 {
        return false;
    }
    return true;
}

fn is_valid_pw_p2(pw: &u32) -> bool {
    let ds = (
        pw / 100000,
        (pw / 10000) % 10,
        (pw / 1000) % 10,
        (pw / 100) % 10,
        (pw / 10) % 10,
        pw % 10,
    );
    if ds.0 > ds.1 || ds.1 > ds.2 || ds.2 > ds.3 || ds.3 > ds.4 || ds.4 > ds.5 {
        return false;
    }
    return (ds.0 == ds.1 && ds.1 != ds.2)
        || (ds.0 != ds.1 && ds.1 == ds.2 && ds.2 != ds.3)
        || (ds.1 != ds.2 && ds.2 == ds.3 && ds.3 != ds.4)
        || (ds.2 != ds.3 && ds.3 == ds.4 && ds.4 != ds.5)
        || (ds.3 != ds.4 && ds.4 == ds.5);
}
fn main() -> io::Result<()> {
    let input_str: Vec<u32> = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .split('-')
    .map(|l| l.parse().unwrap())
    .collect();

    let start = input_str[0];
    let end = input_str[1];
    // Do both parts at once
    let count_both = (start..=end)
        .map(|x| (is_valid_pw(&x), is_valid_pw_p2(&x)))
        .fold((0, 0), |acc, x| (acc.0 + x.0 as u32, acc.1 + x.1 as u32));
    println!("Number of possible passwords: {}", count_both.0);
    println!("Number of possible passwords (part 2): {}", count_both.1);
    Ok(())
}
