use std::env;
use std::fs;
use std::io;

fn hash_alg(s: &str) -> usize {
    s.chars()
        .fold(0usize, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn main() -> io::Result<()> {
    let input_str: String =
        fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;

    let hash_strs: Vec<_> = input_str.trim().split(',').collect();

    let hash_sum: usize = hash_strs.iter().map(|s| hash_alg(s)).sum();

    println!("Sum of string HASHes: {}", hash_sum);

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for s in hash_strs {
        let mut it = s.split(['-', '=']);
        let label = it.next().unwrap();
        let label_hash = hash_alg(label);
        let focal_length = it.next().unwrap();
        if focal_length.is_empty() {
            // '-'
            // remove label from box, if exists
            if let Some(idx) = boxes[label_hash].iter().position(|&(s, _)| s == label) {
                boxes[label_hash].remove(idx);
            }
        } else {
            // '='
            let lens = focal_length.parse().unwrap();
            if let Some(idx) = boxes[label_hash].iter().position(|&(s, _)| s == label) {
                boxes[label_hash][idx].1 = lens;
            } else {
                boxes[label_hash].push((label, lens));
            }
        }
    }

    let focusing_power: usize = boxes
        .iter()
        .enumerate()
        .map(|(box_number, box_)| {
            box_.iter()
                .enumerate()
                .map(|(slot_number, (_, focal_length))| {
                    (box_number + 1) * (slot_number + 1) * focal_length
                })
                .sum::<usize>()
        })
        .sum();

    println!("Focusing power of all lenses: {}", focusing_power);

    Ok(())
}
