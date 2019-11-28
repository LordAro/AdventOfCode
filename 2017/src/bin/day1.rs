use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let mut input = match File::open(&env::args().nth(1).unwrap()) {
        Err(why) => panic!("Could not open input file: {}\n", why),
        Ok(fh) => fh,
    };

    let mut fstr = String::new();
    input.read_to_string(&mut fstr).unwrap();
    let digits: Vec<_> = fstr.trim().chars().filter_map(|c| c.to_digit(10)).collect();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..digits.len() {
        let j = (i + 1) % digits.len();
        let j2 = (i + (digits.len() / 2)) % digits.len();

        let ith = *digits.get(i).unwrap();
        let jth = *digits.get(j).unwrap();
        let j2th = *digits.get(j2).unwrap();

        if ith == jth {
            sum1 += ith;
        }
        if ith == j2th {
            sum2 += ith;
        }
    }
    println!("Captcha1 solution: {}", sum1);
    println!("Captcha2 solution: {}", sum2);
}
