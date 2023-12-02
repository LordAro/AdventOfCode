use std::cmp::max;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

extern crate regex;
use regex::Regex;

struct CubeCount {
    r: u32,
    g: u32,
    b: u32,
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
        .map(|l| {
            re.captures_iter(l)
                .map(|c| c.extract())
                .map(|(_, [num_str, col_str])| {
                    // build list of (num, col) pairs
                    (
                        num_str.parse::<u32>().unwrap(),
                        col_str.chars().next().unwrap(),
                    )
                })
                .fold(CubeCount { r: 0, g: 0, b: 0 }, |acc, d| {
                    // fold list of pairs into max count of each colour as we don't need to store anything else
                    match d.1 {
                        'r' => CubeCount {
                            r: max(acc.r, d.0),
                            g: acc.g,
                            b: acc.b,
                        },
                        'g' => CubeCount {
                            r: acc.r,
                            g: max(acc.g, d.0),
                            b: acc.b,
                        },
                        'b' => CubeCount {
                            r: acc.r,
                            g: acc.g,
                            b: max(acc.b, d.0),
                        },
                        _ => panic!("Unexpected colour!"),
                    }
                })
        })
        .collect();

    let possible_game_id_sum = cube_games
        .iter()
        .enumerate()
        .filter(|(_, cube_count)| cube_count.r <= 12 && cube_count.g <= 13 && cube_count.b <= 14)
        .fold(0, |acc, (id, _)| acc + id + 1); // index to 1-based

    println!("Sum of possible game IDs: {}", possible_game_id_sum);

    let sum_set_power: u32 = cube_games
        .iter()
        .map(|cube_count| cube_count.r * cube_count.g * cube_count.b)
        .sum();

    println!("Sum of game set power: {}", sum_set_power);

    Ok(())
}
