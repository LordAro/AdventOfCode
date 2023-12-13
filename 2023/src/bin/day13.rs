use std::env;
use std::fs;
use std::io;

type Grid = Vec<Vec<bool>>;

fn find_mirror_vert(map: &Grid, target_difference: usize) -> Option<usize> {
    for pivot in 1..map[0].len() {
        let mut count_diffs = 0;
        let x_height = pivot.min(map[0].len() - pivot);
        for x in 0..x_height {
            for y in 0..map.len() {
                if map[y][pivot + x] != map[y][pivot - (x + 1)] {
                    count_diffs += 1;
                }
            }
            if count_diffs > target_difference {
                break;
            }
        }
        if count_diffs == target_difference {
            return Some(pivot);
        }
    }
    None
}

fn find_mirror_horz(map: &Grid, target_difference: usize) -> Option<usize> {
    for pivot in 1..map.len() {
        let mut count_diffs = 0;
        let y_height = pivot.min(map.len() - pivot);
        for y in 0..y_height {
            for x in 0..map[0].len() {
                if map[pivot + y][x] != map[pivot - (y + 1)][x] {
                    count_diffs += 1;
                }
            }
            if count_diffs > target_difference {
                break;
            }
        }
        if count_diffs == target_difference {
            return Some(pivot);
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
