use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn get_route<T: BufRead>(reader: &mut std::io::Lines<T>) -> Vec<(char, i32)> {
    reader
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| {
            (
                s.chars().next().unwrap(),
                i32::from_str_radix(&s[1..], 10).unwrap(),
            )
        })
        .collect()
}

fn get_change_dir(dir: char) -> Option<(i32, i32)> {
    match dir {
        'U' => Some((0, 1)),
        'L' => Some((1, 0)),
        'D' => Some((0, -1)),
        'R' => Some((-1, 0)),
        _ => None,
    }
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines();

    let route1 = get_route(&mut reader);
    let route2 = get_route(&mut reader);

    let mut visited_coords: HashSet<(i32, i32)> = HashSet::new();
    let mut cur_pos = (0, 0);
    for (dir, dist) in &route1 {
        let change = get_change_dir(*dir).unwrap();
        for _ in 0..*dist {
            cur_pos = (cur_pos.0 + change.0, cur_pos.1 + change.1);
            visited_coords.insert(cur_pos);
        }
    }

    cur_pos = (0, 0);
    let mut min_dist = i32::max_value();
    for (dir, dist) in &route2 {
        let change = get_change_dir(*dir).unwrap();
        for _ in 0..*dist {
            cur_pos = (cur_pos.0 + change.0, cur_pos.1 + change.1);
            if visited_coords.contains(&cur_pos) {
                let man_dist = cur_pos.0.abs() + cur_pos.1.abs();
                if man_dist < min_dist {
                    min_dist = man_dist;
                }
            }
        }
    }
    println!("Wires cross at distance: {}", min_dist);
    Ok(())
}
