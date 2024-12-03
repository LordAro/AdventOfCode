use regex::Regex;
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(do\(\))|(don't\(\))").unwrap();

    let mut p1_sum = 0;
    let mut p2_sum = 0;
    let mut capturing = true;
    for c in re.captures_iter(&input) {
        if c.get(3).is_some() || c.get(4).is_some() {
            capturing = c.get(3).is_some();
        } else {
            let a = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
            p1_sum += a * b;
            if capturing {
                p2_sum += a * b;
            }
        }
    }
    println!("P1: Multiplication sum: {p1_sum}");
    println!("P2: Conditional multiplication sum: {p2_sum}");

    Ok(())
}
