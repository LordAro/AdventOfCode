use std::env;
use std::fs;
use std::io;

fn hash_alg(s: &str) -> usize {
    s.chars()
        .fold(0usize, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn place_into_boxes<'a>(strs: &[&'a str]) -> Vec<Vec<(&'a str, usize)>> {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for s in strs {
        match s.split_once('=') {
            None => {
                // '-'
                let label = &s[..s.len() - 1];
                let label_hash = hash_alg(label);
                // remove label from box, if exists
                if let Some(idx) = boxes[label_hash].iter().position(|&(s, _)| s == label) {
                    boxes[label_hash].remove(idx);
                }
            }
            Some((label, focal_length)) => {
                // '='
                let label_hash = hash_alg(label);
                let lens = focal_length.parse().unwrap();
                if let Some(idx) = boxes[label_hash].iter().position(|&(s, _)| s == label) {
                    boxes[label_hash][idx].1 = lens;
                } else {
                    boxes[label_hash].push((label, lens));
                }
            }
        }
    }
    boxes
}

fn calc_focusing_power(boxes: &[Vec<(&str, usize)>]) -> usize {
    boxes
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
        .sum()
}

fn main() -> io::Result<()> {
    let input_str: String =
        fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;

    let hash_strs: Vec<_> = input_str.trim().split(',').collect();

    let hash_sum: usize = hash_strs.iter().map(|s| hash_alg(s)).sum();

    println!("Sum of string HASHes: {}", hash_sum);

    let boxes = place_into_boxes(&hash_strs);

    let focusing_power = calc_focusing_power(&boxes);

    println!("Focusing power of all lenses: {}", focusing_power);

    Ok(())
}
