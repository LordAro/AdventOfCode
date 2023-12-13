use std::cmp;
use std::env;
use std::fs;
use std::io;

type Grid = Vec<Vec<bool>>;

fn find_mirror_vert(map: &Grid, target_difference: usize) -> Option<usize> {
    for pivot in 0..map[0].len() - 1 {
        let mut count_diffs = 0;
        for x in 0..cmp::min(map[0].len() - 1 - (pivot + 1), pivot) + 1 {
            let x_col: Vec<_> = map.iter().map(|l| l[pivot - x]).collect();
            let x2_col: Vec<_> = map.iter().map(|l| l[pivot + 1 + x]).collect();
            for idx in 0..x_col.len() {
                if x_col[idx] != x2_col[idx] {
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
            let y_row = &map[pivot - y];
            let y2_row = &map[pivot + 1 + y];
            for idx in 0..y_row.len() {
                if y_row[idx] != y2_row[idx] {
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

    let mut sum: usize = 0;
    for b in &blocks {
        if let Some(y) = find_mirror_vert(b, 0) {
            sum += y;
        } else if let Some(x) = find_mirror_horz(b, 0) {
            sum += x * 100;
        } else {
            panic!("No match!");
        }
    }

    println!("Mirror line summary: {}", sum);

    let mut smudge_sum: usize = 0;
    for b in &blocks {
        if let Some(y) = find_mirror_vert(b, 1) {
            smudge_sum += y;
        } else if let Some(x) = find_mirror_horz(b, 1) {
            smudge_sum += x * 100;
        } else {
            panic!("No match!");
        }
    }
    println!(
        "Mirror line summary, accounting for smudges: {}",
        smudge_sum
    );
    Ok(())
}
