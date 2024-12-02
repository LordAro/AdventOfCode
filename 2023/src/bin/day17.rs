use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct PathNode {
    cost: u32,
    coord: Coord,
    prev_coord: Coord,
    x_move_count: u8,
    y_move_count: u8,
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .then(self.coord.cmp(&other.coord))
            .then(self.y_move_count.cmp(&other.y_move_count))
            .then(self.x_move_count.cmp(&other.x_move_count))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_adjacents(map: &[Vec<u32>], pos: Coord) -> [Option<Coord>; 4] {
    [
        pos.y.checked_sub(1).map(|y| Coord { x: pos.x, y }), // north
        Some(Coord {
            x: pos.x,
            y: pos.y + 1,
        })
        .filter(|c| c.y < map.len()), // south
        pos.x.checked_sub(1).map(|x| Coord { x, y: pos.y }), // west
        Some(Coord {
            x: pos.x + 1,
            y: pos.y,
        })
        .filter(|c| c.x < map[0].len()), // east
    ]
}

fn get_route<const MIN_LEN: u8, const MAX_LEN: u8>(
    map: &[Vec<u32>],
    source: Coord,
    target: Coord,
) -> u32 {
    let mut to_search = BTreeSet::new();
    to_search.insert(PathNode {
        cost: 0,
        coord: source,
        prev_coord: source,
        x_move_count: 0,
        y_move_count: 0,
    });
    let mut searched: HashSet<(Coord, Coord, u8, u8)> = HashSet::new();

    while let Some(pn) = to_search.pop_first() {
        //println!("{pn:?}");
        if pn.coord == target {
            return pn.cost;
        }

        searched.insert((pn.coord, pn.prev_coord, pn.x_move_count, pn.y_move_count)); // can't go back to different node
        for neighbour_quad in get_adjacents(map, pn.coord)
            .into_iter()
            .flatten()
            // don't go backwards
            .filter(|n| *n != pn.prev_coord)
            .filter(|n| {
                if (pn.prev_coord.x == pn.coord.x && pn.coord.x == n.x)
                    || (pn.prev_coord.y == pn.coord.y && pn.coord.y == n.y)
                {
                    // no change in direction, allowed (might be excluded further down)
                    true
                } else if n.y != pn.coord.y {
                    pn.x_move_count >= MIN_LEN
                } else if n.x != pn.coord.x {
                    pn.y_move_count >= MIN_LEN
                } else {
                    unreachable!()
                }
            })
            .map(|n| {
                let new_x_count = if n.y == pn.coord.y {
                    pn.x_move_count + 1
                } else {
                    0
                };
                let new_y_count = if n.x == pn.coord.x {
                    pn.y_move_count + 1
                } else {
                    0
                };
                (n, pn.coord, new_x_count, new_y_count)
            })
            .filter(|quad| {
                // remove neighbours that are too far in a straight line
                // remove seen before
                quad.2 <= MAX_LEN && quad.3 <= MAX_LEN && !searched.contains(quad)
            })
        {
            //println!(" -> {neighbour_quad:?}");
            let (n, _, x_move_count, y_move_count) = neighbour_quad;
            let new_heat_loss = map[n.y][n.x];
            to_search.insert(PathNode {
                cost: pn.cost + new_heat_loss,
                coord: n,
                prev_coord: pn.coord,
                x_move_count,
                y_move_count,
            });
        }
    }
    unreachable!() // No path!
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;

    let map: Vec<Vec<u32>> = input_str
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let route_cost = get_route::<0, 3>(
        &map,
        Coord { x: 0, y: 0 },
        Coord {
            x: map[0].len() - 1,
            y: map.len() - 1,
        },
    );

    println!("Total route cost: {route_cost}");

    let ultra_route_cost = get_route::<4, 10>(
        &map,
        Coord { x: 0, y: 0 },
        Coord {
            x: map[0].len() - 1,
            y: map.len() - 1,
        },
    );

    println!("Total route cost with ultra crucibles: {ultra_route_cost}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input_str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        let map: Vec<Vec<u32>> = input_str
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let route_cost = get_route::<0, 3>(
            &map,
            Coord { x: 0, y: 0 },
            Coord {
                x: map[0].len() - 1,
                y: map.len() - 1,
            },
        );
        assert_eq!(route_cost, 102);
    }

    #[test]
    fn ex2() {
        let input_str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        let map: Vec<Vec<u32>> = input_str
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let route_cost = get_route::<4, 10>(
            &map,
            Coord { x: 0, y: 0 },
            Coord {
                x: map[0].len() - 1,
                y: map.len() - 1,
            },
        );
        assert_eq!(route_cost, 94);
    }
}
