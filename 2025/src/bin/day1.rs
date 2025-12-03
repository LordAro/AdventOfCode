use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let mut p1_zero_counts = 0;
    let mut p2_zero_counts = 0;
    let mut dial_pos: isize = 50;
    for line in input.lines() {
        let n: isize = line[1..].parse().unwrap();
        let orig_dial_pos = dial_pos;
        let raw_dial_pos = if line.starts_with('L') {
            orig_dial_pos + n
        } else {
            orig_dial_pos - n
        };
        dial_pos = raw_dial_pos.rem_euclid(100);
        if dial_pos == 0 {
            p1_zero_counts += 1;
        }
        let rotations = if raw_dial_pos >= 100 {
            raw_dial_pos / 100
        } else if raw_dial_pos <= 0 && orig_dial_pos > 0 {
            -raw_dial_pos / 100 + 1
        } else if raw_dial_pos <= 0 {
            -raw_dial_pos / 100
        } else {
            0
        };
        p2_zero_counts += rotations;
    }
    println!("P1: Password: {p1_zero_counts}");
    println!("P2: Password 0x434C49434B: {p2_zero_counts}");
    Ok(())
}
