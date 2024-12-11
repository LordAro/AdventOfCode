use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

fn get_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

fn count_trails(grid: &[Vec<u8>], start: Coord) -> (usize, usize) {
    let mut to_search = VecDeque::from([start]);
    let mut trail_ends: Vec<Coord> = vec![];
    while let Some(c) = to_search.pop_front() {
        let cur_level = grid[c.y][c.x];
        if cur_level == b'9' {
            trail_ends.push(c);
            continue;
        }

        for n in [
            c.y.checked_sub(1).map(|y| Coord { x: c.x, y }),
            Some(c.y + 1)
                .filter(|y| *y < grid.len())
                .map(|y| Coord { x: c.x, y }),
            c.x.checked_sub(1).map(|x| Coord { x, y: c.y }),
            Some(c.x + 1)
                .filter(|x| *x < grid[c.y].len())
                .map(|x| Coord { x, y: c.y }),
        ]
        .iter()
        .flatten()
        .filter(|next| grid[next.y][next.x] == cur_level + 1)
        {
            to_search.push_back(*n);
        }
    }
    let total_trails = trail_ends.len();
    trail_ends.sort();
    trail_ends.dedup();
    let unique_trails = trail_ends.len();
    (unique_trails, total_trails)
}

fn get_all_trails(grid: &[Vec<u8>]) -> (usize, usize) {
    let mut unique_trail_sum = 0;
    let mut distinct_trail_sum = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == b'0' {
                let (p1, p2) = count_trails(grid, Coord { x, y });
                unique_trail_sum += p1;
                distinct_trail_sum += p2;
            }
        }
    }
    (unique_trail_sum, distinct_trail_sum)
}

fn main() -> io::Result<()> {
    let input: Vec<Vec<u8>> = get_grid(&fs::read_to_string(
        env::args().nth(1).expect("missing cli argument"),
    )?);

    let (p1, p2) = get_all_trails(&input);
    println!("P1: Unique trail route sum: {p1}");
    println!("P2: Distinct trail route sum: {p2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = get_grid(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );

        assert_eq!(get_all_trails(&input).0, 36);
    }
}
