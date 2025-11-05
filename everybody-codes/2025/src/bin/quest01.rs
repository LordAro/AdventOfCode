use std::cmp;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input: String = fs::read_to_string(p1_input_filename)?;
    let p1_words: Vec<_> = p1_input
        .lines()
        .next()
        .expect("Unexpected input format")
        .split(',')
        .collect();

    let move_instrs: Vec<_> = p1_input
        .lines()
        .nth(2)
        .expect("Unexpected input format")
        .split(',')
        .collect();

    let mut p1_cur_idx: usize = 0;
    for move_instr in move_instrs {
        let n: usize = move_instr[1..].parse().unwrap();
        if move_instr.chars().next() == Some('R') {
            p1_cur_idx = cmp::min(p1_cur_idx + n, p1_words.len() - 1);
        } else {
            p1_cur_idx = p1_cur_idx.saturating_sub(n);
        }
    }

    let p2_input: String = fs::read_to_string(p2_input_filename)?;
    let p2_words: Vec<_> = p2_input
        .lines()
        .next()
        .expect("Unexpected input format")
        .split(',')
        .collect();

    let move_instrs: Vec<_> = p2_input
        .lines()
        .nth(2)
        .expect("Unexpected input format")
        .split(',')
        .collect();

    let mut p2_cur_idx: usize = 0;
    for move_instr in move_instrs {
        let n: usize = move_instr[1..].parse().unwrap();
        if move_instr.chars().next() == Some('R') {
            p2_cur_idx = (p2_cur_idx + n) % p2_words.len();
        } else {
            p2_cur_idx = (p2_cur_idx + p2_words.len() - n) % p2_words.len();
        }
    }

    let p3_input: String = fs::read_to_string(p3_input_filename)?;
    let mut p3_words: Vec<_> = p3_input
        .lines()
        .next()
        .expect("Unexpected input format")
        .split(',')
        .collect();

    let move_instrs: Vec<_> = p3_input
        .lines()
        .nth(2)
        .expect("Unexpected input format")
        .split(',')
        .collect();

    let p3_words_len = p3_words.len();
    for move_instr in move_instrs {
        let n: usize = move_instr[1..].parse().unwrap();
        if move_instr.chars().next() == Some('R') {
            p3_words.swap(0, n % p3_words_len);
        } else {
            p3_words.swap(0, (4 * p3_words_len - n) % p3_words_len);
        }
    }

    println!("P1: Name: {}", p1_words[p1_cur_idx]);
    println!("P2: Name: {}", p2_words[p2_cur_idx]);
    println!("P3: Name: {}", p3_words[0]);
    Ok(())
}
