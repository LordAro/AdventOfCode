use regex::Regex;
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)").unwrap();

    let captures: Vec<_> = re.captures_iter(&input).collect();
    let mut p1_sum = 0;
    let mut p2_sum = 0;
    let mut capturing = true;
    for c in captures {
        if c.get(0).unwrap().as_str() == "do()" {
            capturing = true;
        } else if c.get(0).unwrap().as_str() == "don't()" {
            capturing = false;
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
