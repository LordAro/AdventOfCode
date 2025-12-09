use itertools::Itertools;
use std::env;
use std::fs;
use std::io;

struct Coord {
    x: usize,
    y: usize,
}

fn main() -> io::Result<()> {
    let input: Vec<_> = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Coord {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let p1_max_rect = input
        .iter()
        .combinations(2)
        .map(|ab| {
            let [a, b] = ab[..] else { unreachable!() };
            (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
        })
        .max()
        .unwrap();

    println!("P1: Largest rectangle area: {p1_max_rect}");
    Ok(())
}
