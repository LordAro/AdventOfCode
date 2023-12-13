use std::cmp;
use std::env;
use std::fs;
use std::io;

type Grid = Vec<Vec<bool>>;

fn find_mirror_vert(map: &Grid, target_difference: usize) -> Option<usize> {
    for pivot in 0..map[0].len() - 1 {
        let mut count_diffs = 0;
        for x in 0..cmp::min(map[0].len() - 1 - (pivot + 1), pivot) + 1 {
            for y in 0..map.len() {
                if map[y][pivot - x] != map[y][pivot + 1 + x] {
                    count_diffs += 1;
                }
            }
            if count_diffs > target_difference {
                break;
            }
        }
        if count_diffs == target_difference {
            return Some(pivot + 1); // 1-based
        }
    }
    None
}

fn find_mirror_horz(map: &Grid, target_difference: usize) -> Option<usize> {
    for pivot in 0..map.len() - 1 {
        let mut count_diffs = 0;
        for y in 0..cmp::min(map.len() - 1 - (pivot + 1), pivot) + 1 {
            for x in 0..map[0].len() {
                if map[pivot - y][x] != map[pivot + 1 + y][x] {
                    count_diffs += 1;
                }
            }
            if count_diffs > target_difference {
                break;
            }
        }
        if count_diffs == target_difference {
            return Some(pivot + 1); // 1-based
        }
    }
    None
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
            find_mirror_vert(b, 0)
                .or_else(|| find_mirror_horz(b, 0).map(|n| n * 100))
                .unwrap()
        })
        .sum();

    println!("Mirror line summary: {}", sum);

    let smudge_sum: usize = blocks
        .iter()
        .map(|b| {
            find_mirror_vert(b, 1)
                .or_else(|| find_mirror_horz(b, 1).map(|n| n * 100))
                .unwrap()
        })
        .sum();
    println!(
        "Mirror line summary, accounting for smudges: {}",
        smudge_sum
    );
    Ok(())
}
