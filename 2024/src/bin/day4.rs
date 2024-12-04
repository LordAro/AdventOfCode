use std::env;
use std::fs;
use std::io;

fn find_xmas(grid: &[Vec<u8>]) -> (usize, usize) {
    let xmas = "XMAS".as_bytes();
    let samx = "SAMX".as_bytes();

    let mut xmas_count = 0;
    let mut x_mas_count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // -
            if x <= grid[y].len() - 4 {
                let subword_h: Vec<_> = (0..4).map(|n| grid[y][x + n]).collect();
                if subword_h == xmas || subword_h == samx {
                    xmas_count += 1;
                }
            }
            // |
            if y <= grid.len() - 4 {
                let subword_v: Vec<_> = (0..4).map(|n| grid[y + n][x]).collect();
                if subword_v == xmas || subword_v == samx {
                    xmas_count += 1;
                }
            }
            // \
            if y <= grid.len() - 4 && x <= grid[y].len() - 4 {
                let subword_d1: Vec<_> = (0..4).map(|n| grid[y + n][x + n]).collect();
                if subword_d1 == xmas || subword_d1 == samx {
                    xmas_count += 1;
                }
            }
            // /
            if y >= 4 - 1 && x <= grid[y].len() - 4 {
                let subword_d2: Vec<_> = (0..4).map(|n| grid[y - n][x + n]).collect();
                if subword_d2 == xmas || subword_d2 == samx {
                    xmas_count += 1;
                }
            }
            // P2
            if y >= 1 && x >= 1 && y < grid.len() - 1 && x < grid[y].len() - 1 {
                let submas_d1: Vec<_> = (0..3).map(|n| grid[y + n - 1][x + n - 1]).collect();
                let submas_d2: Vec<_> = (0..3).map(|n| grid[y + 1 - n][x + n - 1]).collect();
                if (submas_d1 == "MAS".as_bytes() || submas_d1 == "SAM".as_bytes())
                    && (submas_d2 == "MAS".as_bytes() || submas_d2 == "SAM".as_bytes())
                {
                    x_mas_count += 1;
                }
            }
        }
    }
    (xmas_count, x_mas_count)
}

fn main() -> io::Result<()> {
    let input: Vec<Vec<u8>> =
        fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
            .lines()
            .map(|l| l.bytes().collect())
            .collect();

    let (xmas_count, x_mas_count) = find_xmas(&input);
    println!("P1: Number of 'XMAS' words: {xmas_count}");
    println!("P2: Number of 'X-MAS' words: {x_mas_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input: Vec<Vec<u8>> = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .lines()
            .map(|l| l.bytes().collect())
            .collect();
        assert_eq!(find_xmas(&input), (18, 9));
    }
}
