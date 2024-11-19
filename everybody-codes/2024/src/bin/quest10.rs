use itertools::iproduct;
use std::fs;
use std::io;

fn try_find_symbol(
    grid: &[Vec<char>],
    grid_x: usize,
    grid_y: usize,
    gx: usize,
    gy: usize,
) -> Option<char> {
    let row: Vec<char> = grid[grid_y + gy]
        .iter()
        .skip(grid_x)
        .take(8)
        .filter(|c| **c != '.')
        .cloned()
        .collect();
    let col: Vec<char> = grid
        .iter()
        .skip(grid_y)
        .take(8)
        .map(|row| row[grid_x + gx])
        .filter(|c| *c != '.')
        .collect();
    let col_possible_match: Vec<_> = col.iter().filter(|c| row.contains(c)).collect();
    if col_possible_match.len() == 1 && *col_possible_match[0] != '?' {
        return Some(*col_possible_match[0]);
    }
    let row_possible_match: Vec<_> = row.iter().filter(|r| col.contains(r)).collect();
    if row_possible_match.len() == 1 && *row_possible_match[0] != '?' {
        return Some(*row_possible_match[0]);
    }
    None
}

fn get_rune_word(grid: &[Vec<char>], grid_x: usize, grid_y: usize) -> Option<String> {
    // try_reduce is nightly only
    iproduct!(2..6, 2..6)
        .map(|(y, x)| grid[grid_y + y][grid_x + x])
        .try_fold(String::new(), |mut acc, ch| {
            if ch == '.' {
                None
            } else {
                acc.push(ch);
                Some(acc)
            }
        })
}

fn get_new_rune_word(grid: &[Vec<char>], grid_x: usize, grid_y: usize) -> String {
    // Only copy the specific grid we're interested in
    // Might be less efficient than just copying the whole thing, dunno
    let specific_grid: Vec<Vec<char>> = grid[grid_y..grid_y + 8]
        .iter()
        .map(|row| row.iter().skip(grid_x).take(8).cloned().collect())
        .collect();
    iproduct!(2..6, 2..6)
        .scan(specific_grid, |updated_grid, (y, x)| {
            if let Some(new_rune) = try_find_symbol(updated_grid, 0, 0, x, y) {
                updated_grid[y][x] = new_rune;
                Some(new_rune)
            } else {
                Some('.')
            }
        })
        .collect()
}

fn get_rune_word_power(rune_word: &str) -> usize {
    rune_word
        .chars()
        .zip(1..)
        .map(|(c, i)| i * (c as u8 - b'A' + 1) as usize)
        .sum()
}

fn find_unique_element(a: &[char], b: &[char]) -> Option<char> {
    for item in a {
        if !b.contains(item) {
            return Some(*item);
        }
    }
    None
}

fn try_place_unknown_symbol(grid: &mut [Vec<char>], grid_x: usize, grid_y: usize) {
    for (y, x) in iproduct!(2..6, 2..6) {
        if grid[grid_y + y][grid_x + x] != '.' {
            continue;
        }
        // So many copies. There's got to be a way to do it without...
        let row: Vec<char> = grid[grid_y + y]
            .iter()
            .skip(grid_x)
            .take(8)
            .cloned()
            .collect();
        let col: Vec<char> = grid
            .iter()
            .skip(grid_y)
            .take(8)
            .map(|row| row[grid_x + x])
            .collect();

        let row_outer: Vec<char> = row
            .iter()
            .take(2)
            .chain(row.iter().skip(6).take(2))
            .cloned()
            .collect();
        let col_outer: Vec<char> = col
            .iter()
            .take(2)
            .chain(col.iter().skip(6).take(2))
            .cloned()
            .collect();
        let row_inner: Vec<char> = row.iter().skip(2).take(4).cloned().collect();
        let col_inner: Vec<char> = col.iter().skip(2).take(4).cloned().collect();
        let row_fill_count = row_inner.iter().filter(|r| **r != '.').count();
        let col_fill_count = col_inner.iter().filter(|c| **c != '.').count();

        if row_fill_count == 3 && col_fill_count == 3 {
            let row_qs = row_outer.iter().filter(|ro| **ro == '?').count();
            let col_qs = col_outer.iter().filter(|co| **co == '?').count();
            if row_qs == 1 && col_qs == 0 {
                let unused_col_item = find_unique_element(&col_outer, &col_inner).unwrap();
                grid[grid_y + y][grid_x + x] = unused_col_item;
                // If we had references I could just assign rather than just look it up again...
                for x in 0..8 {
                    if matches!(grid[grid_y + y][grid_x + x], '?' | '.') {
                        grid[grid_y + y][grid_x + x] = unused_col_item;
                    }
                }
            }
            if col_qs == 1 && row_qs == 0 {
                let unused_row_item = find_unique_element(&row_outer, &row_inner).unwrap();
                for y in 0..8 {
                    if matches!(grid[grid_y + y][grid_x + x], '?' | '.') {
                        grid[grid_y + y][grid_x + x] = unused_row_item;
                    }
                }
            }
        }
    }
}

fn process_merged_grid(full_grid: &[Vec<char>]) -> usize {
    let mut grid = full_grid.to_vec();
    loop {
        let mut made_changes = 0;
        // grids overlap, only increment by 6 and don't try to read the last column
        for (gy, gx) in iproduct!(
            (0..grid.len() - 2).step_by(6),
            (0..grid[0].len() - 2).step_by(6)
        ) {
            let mut made_changes_local = false;
            for (y, x) in iproduct!(2..6, 2..6) {
                if grid[gy + y][gx + x] == '.' {
                    if let Some(r) = try_find_symbol(&grid, gx, gy, x, y) {
                        grid[gy + y][gx + x] = r;
                        made_changes += 1;
                        made_changes_local = true;
                    }
                }
            }
            if made_changes_local {
                try_place_unknown_symbol(&mut grid, gx, gy);
            }
        }
        if made_changes == 0 {
            break;
        }
    }
    let mut power = 0;
    // Determine final words
    for (gy, gx) in iproduct!(
        (0..grid.len() - 2).step_by(6),
        (0..grid[0].len() - 2).step_by(6)
    ) {
        if let Some(rune) = get_rune_word(&grid, gx, gy) {
            power += get_rune_word_power(&rune);
        }
    }
    power
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input: Vec<Vec<_>> = fs::read_to_string(p1_input_filename)?
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    println!("P1: New rune word: {}", get_new_rune_word(&p1_input, 0, 0));

    let p2_all_grids: Vec<Vec<_>> = fs::read_to_string(p2_input_filename)?
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    // step of 9 because separated by a space
    let p2_power: usize = iproduct!(
        (0..p2_all_grids.len()).step_by(9),
        (0..p2_all_grids[0].len()).step_by(9)
    )
    .map(|(y, x)| {
        let r = get_new_rune_word(&p2_all_grids, x, y);
        get_rune_word_power(&r)
    })
    .sum();
    println!("P2: Total power: {p2_power}");

    let p3_all_grids: Vec<Vec<_>> = fs::read_to_string(p3_input_filename)?
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let p3_power = process_merged_grid(&p3_all_grids);
    println!("P3: Total power: {p3_power}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = [
            "**PCBS**".chars().collect::<Vec<_>>(),
            "**RLNW**".chars().collect::<Vec<_>>(),
            "BV....PT".chars().collect::<Vec<_>>(),
            "CR....HZ".chars().collect::<Vec<_>>(),
            "FL....JW".chars().collect::<Vec<_>>(),
            "SG....MN".chars().collect::<Vec<_>>(),
            "**FTZV**".chars().collect::<Vec<_>>(),
            "**GMJH**".chars().collect::<Vec<_>>(),
        ];

        assert_eq!(try_find_symbol(&input, 0, 0, 2, 2), Some('P'));
        assert_eq!(get_new_rune_word(&input, 0, 0), "PTBVRCZHFLJWGMNS");
    }

    #[test]
    fn ex2() {
        assert_eq!(get_rune_word_power("PTBVRCZHFLJWGMNS"), 1851);
    }

    #[test]
    fn ex3() {
        let input = "**XFZB**DCST**
**LWQK**GQJH**
?G....WL....DQ
BS....H?....CN
P?....KJ....TV
NM....Z?....SG
**NSHM**VKWZ**
**PJGV**XFNL**
WQ....?L....YS
FX....DJ....HV
?Y....WM....?J
TJ....YK....LP
**XRTK**BMSP**
**DWZN**GCJV**";
        let input_arr: Vec<Vec<_>> = input.lines().map(|r| r.chars().collect()).collect();
        let power = process_merged_grid(&input_arr);
        assert_eq!(power, 3889);
    }
}
