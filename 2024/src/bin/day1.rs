use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            l.split_once("   ")
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap()
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    let total_distance: usize = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    println!("P1: Total distance between lists: {total_distance}");

    let similarity_score: usize = left_list
        .iter()
        .map(|l| {
            // TODO: Should be a better way of doing this given a sorted list
            let count = right_list.iter().filter(|r| *r == l).count();
            l * count
        })
        .sum();

    println!("P2: Total similarity score: {similarity_score}");
    Ok(())
}
