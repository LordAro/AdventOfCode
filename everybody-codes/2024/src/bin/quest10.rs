use itertools::iproduct;
use std::fs;
use std::io;

fn get_new_rune(grid: &[Vec<char>], x: usize, y: usize) -> char {
    let row: Vec<_> = grid[y].iter().filter(|c| **c != '.').collect();
    grid.iter()
        .map(|row| row[x])
        .filter(|c| *c != '.')
        .find(|c| row.contains(&c))
        .unwrap()
}

fn get_new_rune_word(grid: &[Vec<char>], grid_x: usize, grid_y: usize) -> String {
    // Only copy the specific grid we're interested in
    // Might be less efficient than just copying the whole thing, dunno
    let mut specific_grid: Vec<Vec<char>> = grid[grid_y..grid_y + 8]
        .iter()
        .map(|row| row.iter().skip(grid_x).take(8).cloned().collect())
        .collect();
    iproduct!(2..6, 2..6)
        .scan(specific_grid, |updated_grid, (y, x)| {
            let new_rune = get_new_rune(&updated_grid, x, y);
            updated_grid[y][x] = new_rune;
            Some(new_rune)
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

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, _p3_input_filename) =
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
    let p2_power: usize = iproduct!(
        (0..p2_all_grids.len()).step_by(9),
        (0..p2_all_grids[0].len()).step_by(9)
    )
    .map(|(y, x)| {
        let r = get_new_rune_word(&p2_all_grids, x, y);
        let p = get_rune_word_power(&r);
        p
    })
    .sum();
    println!("P2: Total power: {p2_power}");
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

        assert_eq!(get_new_rune(&input, 2, 2), 'P');
        assert_eq!(get_new_rune_word(&input, 0, 0), "PTBVRCZHFLJWGMNS");
    }

    #[test]
    fn ex2() {
        assert_eq!(get_rune_word_power("PTBVRCZHFLJWGMNS"), 1851);
    }
}
