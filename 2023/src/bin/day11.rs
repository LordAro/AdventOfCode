use itertools::Itertools;
use std::env;
use std::fs;
use std::io;

#[derive(Clone)]
struct Coord {
    x: usize,
    y: usize,
}

fn expand_universe<const N: usize>(galaxies: &[Coord]) -> impl Iterator<Item = Coord> + '_ {
    let mut seen_xs = vec![];
    let mut seen_ys = vec![];
    for c in galaxies {
        seen_xs.push(c.x);
        seen_ys.push(c.y);
    }
    seen_xs.sort();
    seen_ys.sort();
    seen_xs.dedup();
    seen_ys.dedup();

    galaxies.iter().map(move |c| {
        let x_idx = seen_xs.partition_point(|&x| x < c.x);
        let y_idx = seen_ys.partition_point(|&y| y < c.y);
        let x_diff = seen_xs[x_idx] - x_idx;
        let y_diff = seen_ys[y_idx] - y_idx;
        Coord {
            x: c.x + x_diff * (N - 1),
            y: c.y + y_diff * (N - 1),
        }
    })
}

fn manhattan(a: &Coord, b: &Coord) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;
    let galaxies: Vec<_> = input_str
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| Coord { x, y })
        })
        .collect();

    let galaxy_distance_sum: usize = expand_universe::<2>(&galaxies)
        .combinations(2)
        .map(|pair| manhattan(&pair[0], &pair[1]))
        .sum();

    println!(
        "Some of distances between expanded galaxies: {}",
        galaxy_distance_sum
    );

    let bigger_galaxy_distance_sum: usize = expand_universe::<1_000_000>(&galaxies)
        .combinations(2)
        .map(|pair| manhattan(&pair[0], &pair[1]))
        .sum();

    println!(
        "Some of distances between even more expanded galaxies: {}",
        bigger_galaxy_distance_sum
    );
    Ok(())
}
