use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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
        (c.y + 1 < grid.len()).then_some(Coord { x: c.x, y: c.y + 1 }),
        c.x.checked_sub(1).map(|x| Coord { x, y: c.y }),
        (c.x + 1 < grid[c.y].len()).then_some(Coord { x: c.x + 1, y: c.y }),
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

fn get_num_sides(grid: &[Vec<u8>], group: &HashSet<Coord>) -> usize {
    let edge_pieces: HashSet<_> = group
        .iter()
        .filter(|c| {
            coord_neighbours(grid, **c)
                .filter(|n| group.contains(n))
                .count()
                != 4
        })
        .collect();

    // sorting order (y,x) vs (x,y) is important to get the groups correct
    let upward_edges: Vec<_> = edge_pieces
        .iter()
        .filter(|c| c.y == 0 || !group.contains(&Coord { x: c.x, y: c.y - 1 }))
        .sorted_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)))
        .collect();
    let leftward_edges: Vec<_> = edge_pieces
        .iter()
        .filter(|c| c.x == 0 || !group.contains(&Coord { x: c.x - 1, y: c.y }))
        .sorted_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
        .collect();
    let rightward_edges: Vec<_> = edge_pieces
        .iter()
        .filter(|c| !group.contains(&Coord { x: c.x + 1, y: c.y }))
        .sorted_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
        .collect();
    let downward_edges: Vec<_> = edge_pieces
        .iter()
        .filter(|c| !group.contains(&Coord { x: c.x, y: c.y + 1 }))
        .sorted_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)))
        .collect();

    let num_upward_edges = upward_edges
        .chunk_by(|a, b| a.x + 1 == b.x && a.y == b.y)
        .count();
    let num_downward_edges = downward_edges
        .chunk_by(|a, b| a.x + 1 == b.x && a.y == b.y)
        .count();
    let num_leftward_edges = leftward_edges
        .chunk_by(|a, b| a.y + 1 == b.y && a.x == b.x)
        .count();
    let num_rightward_edges = rightward_edges
        .chunk_by(|a, b| a.y + 1 == b.y && a.x == b.x)
        .count();

    num_upward_edges + num_downward_edges + num_leftward_edges + num_rightward_edges
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
    let discount_fence_price: usize = groups
        .iter()
        .map(|g| get_num_sides(&grid, g) * g.len())
        .sum();
    println!("P2: Total fence price: {discount_fence_price}");

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

    #[test]
    fn ex2() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        let grid = parse_grid(input);
        let groups = get_groups(&grid);
        let sides: Vec<_> = groups.iter().map(|g| get_num_sides(&grid, g)).collect();
        assert_eq!(sides, [4, 4, 8, 4, 4]);
    }

    #[test]
    fn ex2b() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let grid = parse_grid(input);
        let groups = get_groups(&grid);
        let sides: Vec<_> = groups.iter().map(|g| get_num_sides(&grid, g)).collect();
        assert_eq!(sides, [12, 4, 4]);
    }

    #[test]
    fn ex2c() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let grid = parse_grid(input);
        let groups = get_groups(&grid);
        let sides: Vec<_> = groups.iter().map(|g| get_num_sides(&grid, g)).collect();
        assert_eq!(sides, [12, 4, 4]);
    }

    #[test]
    fn ex2d() {
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
        let sides: Vec<_> = groups.iter().map(|g| get_num_sides(&grid, g)).collect();
        assert_eq!(sides, [10, 4, 22, 12, 10, 12, 4, 8, 16, 6, 6]);
    }
}
