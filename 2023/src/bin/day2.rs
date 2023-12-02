use std::cmp::max;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct CubeGame {
    id: usize,
    draws: Vec<(char, u32)>,
}

fn get_max_cube_counts(cg: &CubeGame) -> (u32, u32, u32) {
    cg.draws.iter().fold((0, 0, 0), |acc, d| {
        if d.0 == 'r' {
            (max(acc.0, d.1), acc.1, acc.2)
        } else if d.0 == 'g' {
            (acc.0, max(acc.1, d.1), acc.2)
        } else if d.0 == 'b' {
            (acc.0, acc.1, max(acc.2, d.1))
        } else {
            panic!("Unexpected colour!");
        }
    })
}

fn main() -> io::Result<()> {
    let input_data: Vec<String> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| l.unwrap().parse().unwrap())
    .collect();

    let re = Regex::new(r"([0-9]+) ([brg])").unwrap();

    let cube_games: Vec<_> = input_data
        .iter()
        .enumerate()
        .map(|(id, l)| CubeGame {
            id: id + 1,
            draws: re
                .captures_iter(&l)
                .map(|c| c.extract())
                .map(|(_, [num_str, col_str])| {
                    (
                        col_str.chars().next().unwrap(),
                        num_str.parse::<u32>().unwrap(),
                    )
                })
                .collect(),
        })
        .collect();

    let possible_games: Vec<_> = cube_games
        .iter()
        .filter(|cg| {
            let max_cube_counts = get_max_cube_counts(cg);
            max_cube_counts.0 <= 12 && max_cube_counts.1 <= 13 && max_cube_counts.2 <= 14
        })
        .collect();

    let sum_possible_ids = possible_games.iter().fold(0, |acc, cg| acc + cg.id);

    println!("Sum of possible game IDs: {}", sum_possible_ids);

    let sum_set_power: u32 = cube_games
        .iter()
        .map(|cg| {
            let max_cube_counts = get_max_cube_counts(cg);
            max_cube_counts.0 * max_cube_counts.1 * max_cube_counts.2
        })
        .sum();

    println!("Sum of game set power: {}", sum_set_power);

    Ok(())
}
