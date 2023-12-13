use std::env;
use std::fs;
use std::io;

extern crate itertools;
use itertools::Itertools;

type Grid = Vec<Vec<bool>>;

fn find_mirror_vert<const N: usize>(map: &Grid) -> Option<usize> {
    (1..map[0].len()).find(|&pivot| {
        let x_height = pivot.min(map[0].len() - pivot);
        (0..map.len())
            .cartesian_product(0..x_height)
            .filter(|&(y, x)| map[y][pivot + x] != map[y][pivot - (x + 1)])
            .count()
            == N
    })
}

fn find_mirror_horz<const N: usize>(map: &Grid) -> Option<usize> {
    (1..map.len()).find(|&pivot| {
        let y_height = pivot.min(map.len() - pivot);
        (0..y_height)
            .cartesian_product(0..map[0].len())
            .filter(|&(y, x)| map[pivot + y][x] != map[pivot - (y + 1)][x])
            .count()
            == N
    })
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;
    let blocks: Vec<Grid> = input_str
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect();

    let sum: usize = blocks
        .iter()
        .map(|b| {
            find_mirror_vert::<0>(b)
                .or_else(|| find_mirror_horz::<0>(b).map(|n| n * 100))
                .unwrap()
        })
        .sum();

    println!("Mirror line summary: {}", sum);

    let smudge_sum: usize = blocks
        .iter()
        .map(|b| {
            find_mirror_vert::<1>(b)
                .or_else(|| find_mirror_horz::<1>(b).map(|n| n * 100))
                .unwrap()
        })
        .sum();
    println!(
        "Mirror line summary, accounting for smudges: {}",
        smudge_sum
    );
    Ok(())
}
