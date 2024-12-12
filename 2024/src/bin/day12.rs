use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

fn coord_neighbours(grid: &[Vec<u8>], c: Coord) -> impl Iterator<Item = Coord> {
    [
        c.y.checked_sub(1).map(|y| Coord { x: c.x, y }),
        Some(c.y + 1)
            .filter(|y| *y < grid.len())
            .map(|y| Coord { x: c.x, y }),
        c.x.checked_sub(1).map(|x| Coord { x, y: c.y }),
        Some(c.x + 1)
            .filter(|x| *x < grid[c.y].len())
            .map(|x| Coord { x, y: c.y }),
    ]
    .into_iter()
    .flatten()
}

fn get_group(grid: &[Vec<u8>], start: Coord) -> HashSet<Coord> {
    let start_char = grid[start.y][start.x];
    let mut to_search = vec![start];
    let mut group_coords = HashSet::from([start]);
    while let Some(c) = to_search.pop() {
        group_coords.insert(c);

        for n in coord_neighbours(grid, c)
            .filter(|n| grid[n.y][n.x] == start_char && !group_coords.contains(n))
        {
            to_search.push(n);
        }
    }
    group_coords
}

fn get_groups(grid: &[Vec<u8>]) -> Vec<HashSet<Coord>> {
    let mut all_groups = vec![];
    let mut seen_coords: HashSet<Coord> = HashSet::with_capacity(grid.len() * grid[0].len());
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = Coord { x, y };
            if !seen_coords.contains(&c) {
                let new_group = get_group(grid, c);
                seen_coords.extend(new_group.iter());
                all_groups.push(new_group);
            }
        }
    }
    all_groups
}

fn get_perimeter(grid: &[Vec<u8>], group: &HashSet<Coord>) -> usize {
    group
        .iter()
        .map(|c| {
            4 - coord_neighbours(grid, *c)
                .filter(|n| group.contains(n))
                .count()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let grid = parse_grid(&fs::read_to_string(
        env::args().nth(1).expect("missing cli argument"),
    )?);

    let groups = get_groups(&grid);
    let fence_price: usize = groups
        .iter()
        .map(|g| get_perimeter(&grid, g) * g.len())
        .sum();
    println!("P1: Total fence price: {fence_price}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        let grid = parse_grid(input);
        let groups = get_groups(&grid);
        let fence_price: usize = groups
            .iter()
            .map(|g| get_perimeter(&grid, g) * g.len())
            .sum();
        assert_eq!(fence_price, 140);
    }

    #[test]
    fn ex1b() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let grid = parse_grid(input);
        let groups = get_groups(&grid);
        let fence_price: usize = groups
            .iter()
            .map(|g| get_perimeter(&grid, g) * g.len())
            .sum();
        assert_eq!(fence_price, 772);
    }

    #[test]
    fn ex1c() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let grid = parse_grid(input);
        let groups = get_groups(&grid);
        let fence_price: usize = groups
            .iter()
            .map(|g| get_perimeter(&grid, g) * g.len())
            .sum();
        assert_eq!(fence_price, 1930);
    }
}
