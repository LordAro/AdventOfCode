use std::fs::File;
use std::env;
use std::io::{self, Write, Read};
use std::process::exit;

fn main() {
    if env::args().count() != 2 {
        io::stderr().write(b"Incorrect number of arguments provided\n").unwrap();
        exit(1);
    }
    let mut input = match File::open(&env::args().nth(1).unwrap()) {
        Err(why) => panic!("Could not open input file: {}\n", why),
        Ok(fh) => fh,
    };

    let mut fstr = String::new();
    input.read_to_string(&mut fstr).unwrap();
    let dirs: Vec<_> = fstr.split(',').map(|dir| dir.trim()).collect();

    let mut cur_dir = 0; // North
    let mut locs = vec![(0, 0)]; // Starting point
    for dir in &dirs {
        cur_dir = match dir.chars().nth(0).unwrap() {
            'L' => (cur_dir + (4 - 1)) % 4,
            'R' => (cur_dir + 1) % 4,
            _ => panic!("Weird turning direction: {:?}\n", dir.chars().nth(0)),
        };
        let dist: i32 = dir.chars().skip(1).collect::<String>().parse().unwrap();
        let old_pos = locs.last().unwrap();
        let mut cur_pos = match cur_dir {
            0 => (old_pos.0 + dist, old_pos.1),
            1 => (old_pos.0, old_pos.1 + dist),
            2 => (old_pos.0 - dist, old_pos.1),
            3 => (old_pos.0, old_pos.1 - dist),
            _ => panic!("Current direction is not a direction: {}", cur_dir),
        };
        let tmp = &mut locs;
        tmp.push(cur_pos);
    }
    let last = locs.last().unwrap();
    let abs = last.0.abs() + last.1.abs();
    println!("Final distance: {} blocks", abs);
    println!("Visited twice: {} blocks", abs);
}
