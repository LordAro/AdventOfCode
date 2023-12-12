use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::iter::successors;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

fn get_next_coord(map: &Vec<Vec<char>>, c: &Coord, d: &Dir) -> Option<Coord> {
    match d {
        Dir::North => {
            if c.y > 0 {
                Some(Coord { x: c.x, y: c.y - 1 })
            } else {
                None
            }
        }
        Dir::East => {
            if c.x < map[0].len() - 1 {
                Some(Coord { x: c.x + 1, y: c.y })
            } else {
                None
            }
        }
        Dir::South => {
            if c.y < map.len() - 1 {
                Some(Coord { x: c.x, y: c.y + 1 })
            } else {
                None
            }
        }
        Dir::West => {
            if c.x > 0 {
                Some(Coord { x: c.x - 1, y: c.y })
            } else {
                None
            }
        }
    }
}

fn get_next_coord_on_loop(map: &Vec<Vec<char>>, pos: &Coord, dir: &Dir) -> Option<(Coord, Dir)> {
    if let Some(next_coord) = get_next_coord(map, pos, dir) {
        let next_dir = match (map[next_coord.y][next_coord.x], dir) {
            // no change, if we hit 'S' we're done, don't care about the direction anymore
            ('-', Dir::East) => Dir::East,
            ('-', Dir::West) => Dir::West,
            ('|', Dir::North) => Dir::North,
            ('|', Dir::South) => Dir::South,
            ('J', Dir::South) => Dir::West,
            ('J', Dir::East) => Dir::North,
            ('F', Dir::North) => Dir::East,
            ('F', Dir::West) => Dir::South,
            ('L', Dir::South) => Dir::East,
            ('L', Dir::West) => Dir::North,
            ('7', Dir::North) => Dir::West,
            ('7', Dir::East) => Dir::South,
            _ => return None,
        };
        Some((next_coord, next_dir))
    } else {
        None
    }
}

fn trace_route(map: &Vec<Vec<char>>, route: &[(Coord, Dir)]) -> (HashSet<Coord>, HashSet<Coord>) {
    let route_set: HashSet<_> = route.iter().map(|(c, _)| c).collect(); // quick lookups
    let mut set_a: HashSet<Coord> = HashSet::new();
    let mut set_b: HashSet<Coord> = HashSet::new();

    for (c, move_dir) in route {
        let (look_dir_a, look_dir_b) = match move_dir {
            Dir::North => (Dir::West, Dir::East),
            Dir::East => (Dir::North, Dir::South),
            Dir::South => (Dir::East, Dir::West),
            Dir::West => (Dir::South, Dir::North),
        };

        if let Some(look_coord_a) = get_next_coord(map, c, &look_dir_a) {
            if !route_set.contains(&look_coord_a) {
                set_a.insert(look_coord_a);
            }
        }
        if let Some(look_coord_b) = get_next_coord(map, c, &look_dir_b) {
            if !route_set.contains(&look_coord_b) {
                set_b.insert(look_coord_b);
            }
        }
    }
    (set_a, set_b)
}

fn flood_fill(
    map: &Vec<Vec<char>>,
    route_set: &HashSet<&Coord>,
    starting_set: &HashSet<Coord>,
) -> HashSet<Coord> {
    let mut to_search: Vec<Coord> = starting_set.iter().copied().collect();
    let mut searched = HashSet::new();

    while let Some(next) = to_search.pop() {
        if route_set.contains(&next) {
            continue;
        }

        if !searched.insert(next) {
            // already searched
            continue;
        }

        to_search.extend(
            [Dir::North, Dir::East, Dir::South, Dir::West]
                .iter()
                .filter_map(|d| get_next_coord(map, &next, d))
                .filter(|c| !route_set.contains(c)),
        );
    }

    searched
}

fn get_enclosed_pieces(map: &Vec<Vec<char>>, route: &[(Coord, Dir)]) -> HashSet<Coord> {
    let (set_a, set_b) = trace_route(map, route);
    debug_assert!(set_a.intersection(&set_b).count() == 0);

    let route_set: HashSet<_> = route.iter().map(|(c, _)| c).collect(); // quick lookups
    let filled_set_a = flood_fill(map, &route_set, &set_a);
    let filled_set_b = flood_fill(map, &route_set, &set_b);

    debug_assert!(filled_set_a.intersection(&filled_set_b).count() == 0);

    if filled_set_a
        .iter()
        .any(|c| c.x == 0 || c.y == 0 || c.x == map[0].len() - 1 || c.y == map.len() - 1)
    {
        // found an edge piece, must be the other set
        filled_set_b
    } else {
        filled_set_a
    }
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;
    let map: Vec<Vec<_>> = input_str.lines().map(|l| l.chars().collect()).collect();

    let start_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| l.iter().position(|&c| c == 'S').map(|x| Coord { x, y }))
        .unwrap();

    // look around to find a suitable starting location
    let (_, start_dir) = [Dir::North, Dir::East, Dir::South, Dir::West]
        .iter()
        .find_map(|d| get_next_coord_on_loop(&map, &start_pos, d))
        .unwrap();

    let route: Vec<_> = successors(Some((start_pos, start_dir)), |(pos, dir)| {
        get_next_coord_on_loop(&map, pos, dir)
    })
    .collect();

    let max_distance = (route.len() + 1) / 2;
    println!("Maximum distance from start: {}", max_distance);

    let enclosed_tiles = get_enclosed_pieces(&map, &route);
    println!("Number of enclosed tiles: {}", enclosed_tiles.len());

    Ok(())
}
