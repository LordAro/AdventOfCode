use std::fs::File;
use std::env;
use std::io::Read;

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let mut input = match File::open(&env::args().nth(1).unwrap()) {
        Err(why) => panic!("Could not open input file: {}\n", why),
        Ok(fh) => fh,
    };

    let mut fstr = String::new();
    input.read_to_string(&mut fstr).unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..fstr.len() {
        let j = (i + 1) % fstr.len();
        let j2 = (i + (fstr.len() / 2)) % fstr.len();

        let ith = fstr.chars().nth(i).unwrap().to_digit(10).unwrap();
        let jth = fstr.chars().nth(j).unwrap().to_digit(10).unwrap();
        let j2th = fstr.chars().nth(j2).unwrap().to_digit(10).unwrap();

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
