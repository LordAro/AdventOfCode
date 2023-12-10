use std::env;
use std::fs;
use std::io;
use std::iter::successors;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

fn get_next_coord(map: &Vec<Vec<char>>, pos: &Coord, dir: Dir) -> Option<(Coord, Dir)> {
    let next_coord = match dir {
        Dir::North => Coord {
            x: pos.x,
            y: pos.y - 1,
        },
        Dir::East => Coord {
            x: pos.x + 1,
            y: pos.y,
        },
        Dir::South => Coord {
            x: pos.x,
            y: pos.y + 1,
        },
        Dir::West => Coord {
            x: pos.x - 1,
            y: pos.y,
        },
    };

    // Let underflow handle any out of bounds accesses (shouldn't ever happen)
    if next_coord.y >= map.len() || next_coord.x >= map[0].len() {
        return None;
    }

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
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;
    let map: Vec<Vec<_>> = input_str.lines().map(|l| l.chars().collect()).collect();

    let start_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| l.iter().position(|&c| c == 'S').map(|x| Coord { x, y }))
        .unwrap();

    let (_, start_dir) = [Dir::North, Dir::East, Dir::South, Dir::West]
        .iter()
        .find_map(|&d| get_next_coord(&map, &start_pos, d))
        .unwrap();

    let route: Vec<_> = successors(Some((start_pos, start_dir)), |(pos, dir)| {
        get_next_coord(&map, &pos, *dir)
    })
    .map(|(pos, _)| pos)
    .collect();

    let max_distance = (route.len() + 1) / 2;
    println!("Maximum distance from start: {}", max_distance);

    Ok(())
}
