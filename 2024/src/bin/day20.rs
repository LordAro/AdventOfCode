use rustc_hash::FxHashSet;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

fn parse_grid(input: &str) -> (FxHashSet<Coord>, Coord, Coord) {
    let mut start = None;
    let mut end = None;
    let mut map = FxHashSet::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord { x, y };
            if c == 'S' {
                start = Some(coord);
            } else if c == 'E' {
                end = Some(coord);
            } else if c == '#' {
                map.insert(coord);
            }
        }
    }
    (map, start.unwrap(), end.unwrap())
}

fn coord_neighbours(max_bound: Coord, c: Coord) -> impl Iterator<Item = Coord> {
    [
        c.y.checked_sub(1).map(|y| Coord { x: c.x, y }),
        (c.y < max_bound.y).then_some(Coord { x: c.x, y: c.y + 1 }),
        c.x.checked_sub(1).map(|x| Coord { x, y: c.y }),
        (c.x < max_bound.x).then_some(Coord { x: c.x + 1, y: c.y }),
    ]
    .into_iter()
    .flatten()
}

fn wall_has_spaces_x(max_bound: Coord, walls: &FxHashSet<Coord>, c: Coord) -> bool {
    let left = c.x.checked_sub(1).map(|x| Coord { x, y: c.y });
    let right = (c.x < max_bound.x).then_some(Coord { x: c.x + 1, y: c.y });
    left.is_some_and(|l| !walls.contains(&l)) && right.is_some_and(|r| !walls.contains(&r))
}

fn wall_has_spaces_y(max_bound: Coord, walls: &FxHashSet<Coord>, c: Coord) -> bool {
    let up = c.y.checked_sub(1).map(|y| Coord { x: c.x, y });
    let down = (c.y < max_bound.y).then_some(Coord { x: c.x, y: c.y + 1 });
    up.is_some_and(|u| !walls.contains(&u)) && down.is_some_and(|d| !walls.contains(&d))
}

fn get_path_len(
    max_bound: Coord,
    walls: &FxHashSet<Coord>,
    start_pos: Coord,
    end_pos: Coord,
    ignored_wall: Option<Coord>,
) -> Option<usize> {
    let mut to_search = BTreeSet::from([(0, start_pos)]);
    let mut searched: FxHashSet<Coord> = FxHashSet::default();
    while let Some((node_dist, node)) = to_search.pop_first() {
        if node == end_pos {
            return Some(node_dist);
        }

        searched.insert(node);

        for n in coord_neighbours(max_bound, node).filter(|n| {
            !searched.contains(n) && (ignored_wall.is_some_and(|iw| iw == *n) || !walls.contains(n))
        }) {
            to_search.insert((node_dist + 1, n));
        }
    }

    None
}

fn main() -> io::Result<()> {
    let (walls, start_pos, end_pos) = parse_grid(&fs::read_to_string(
        env::args().nth(1).expect("missing cli argument"),
    )?);

    let max_x = walls.iter().max_by_key(|c| c.x).unwrap().x;
    let max_y = walls.iter().max_by_key(|c| c.y).unwrap().y;
    let max_bound = Coord { x: max_x, y: max_y };
    let initial_route_length = get_path_len(max_bound, &walls, start_pos, end_pos, None).unwrap();

    let possible_cheat_locations: Vec<_> = walls
        .iter()
        .filter(|w| {
            wall_has_spaces_x(max_bound, &walls, **w) || wall_has_spaces_y(max_bound, &walls, **w)
        })
        .collect();

    let mut count_100ps_saving = 0;
    for cheat_loc in &possible_cheat_locations {
        let new_route_length =
            get_path_len(max_bound, &walls, start_pos, end_pos, Some(**cheat_loc)).unwrap();
        let saving = initial_route_length - new_route_length;
        if saving >= 100 {
            count_100ps_saving += 1;
        }
    }
    println!("P1: Number of cheats saving at least 100ps: {count_100ps_saving}");
    Ok(())
}
