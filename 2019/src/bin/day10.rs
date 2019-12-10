use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a.rem_euclid(b))
    }
}

fn main() {
    let asteroids: HashSet<_> = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .filter_map(|r| r.ok())
    .enumerate()
    .flat_map(|(i, v)| {
        v.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(move |(j, _)| (i as isize, j as isize))
            .collect::<Vec<_>>()
    })
    .collect();

    let best_loc = asteroids
        .iter()
        .map(|s| {
            let source = *s;
            let seen: HashSet<_> = asteroids
                .iter()
                .filter(|&t| s != t)
                .map(|t| {
                    let target = *t;
                    let normalised = (target.0 - source.0, target.1 - source.1);
                    let gcd = gcd(normalised.0, normalised.1);
                    let mut line = source;
                    loop {
                        line = (
                            (line.0 + (normalised.0 / gcd)),
                            (line.1 + (normalised.1 / gcd)),
                        );
                        if asteroids.contains(&line) {
                            break;
                        }
                    }
                    line
                })
                .collect();
            (source, seen.len())
        })
        .max_by_key(|(_, count)| *count)
        .unwrap();
    println!(
        "Best place for monitoring station is at {:?} which can see {} asteroids",
        best_loc.0, best_loc.1
    );
}
