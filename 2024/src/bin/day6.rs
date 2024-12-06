use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

fn parse_input(input: &str) -> (HashSet<Coord>, Coord, (Coord, Dir)) {
    let mut obstacles = HashSet::default();
    let mut max_coord = Coord { x: 0, y: 0 };
    let mut pos = Coord { x: 0, y: 0 };
    for (y_idx, line) in input.lines().enumerate() {
        let y = y_idx.try_into().unwrap();
        for (x_idx, c) in line.chars().enumerate() {
            let x = x_idx.try_into().unwrap();
            match c {
                '#' => _ = obstacles.insert(Coord { x, y }),
                '^' => pos = Coord { x, y },
                _ => (),
            }
            max_coord = Coord { x, y }; // last coord is largest
        }
    }
    (obstacles, max_coord, (pos, Dir::Up))
}

fn get_route_locations(
    obstacles: &HashSet<Coord>,
    max_coord: Coord,
    guard_posdir: (Coord, Dir),
) -> (HashSet<Coord>, bool) {
    let mut positions: HashSet<(Coord, Dir)> = HashSet::default();
    let mut guard_pos = guard_posdir.0;
    let mut guard_dir = guard_posdir.1;
    let mut loop_detected = false;
    while guard_pos.x >= 0
        && guard_pos.y >= 0
        && guard_pos.x <= max_coord.x
        && guard_pos.y <= max_coord.y
    {
        loop_detected = !positions.insert((guard_pos, guard_dir));
        if loop_detected {
            break;
        }
        let next_pos = match guard_dir {
            Dir::Up => Coord {
                x: guard_pos.x,
                y: guard_pos.y - 1,
            },
            Dir::Right => Coord {
                x: guard_pos.x + 1,
                y: guard_pos.y,
            },
            Dir::Down => Coord {
                x: guard_pos.x,
                y: guard_pos.y + 1,
            },
            Dir::Left => Coord {
                x: guard_pos.x - 1,
                y: guard_pos.y,
            },
        };

        if obstacles.contains(&next_pos) {
            guard_dir = guard_dir.turn_right();
        } else {
            guard_pos = next_pos;
        }
    }
    let unique_positions = positions.iter().map(|(pos, _)| *pos).collect();
    (unique_positions, loop_detected)
}

fn get_num_possible_loops(
    initial_obstacles: &HashSet<Coord>,
    max_coord: Coord,
    initial_guard_posdir: (Coord, Dir),
    possible_obstacle_locations: &HashSet<Coord>,
) -> usize {
    let mut loop_count = 0;
    for pos in possible_obstacle_locations {
        if *pos == initial_guard_posdir.0 {
            continue;
        }
        let mut new_obstacles = initial_obstacles.clone();
        new_obstacles.insert(*pos);
        let (_tmp, is_loop) = get_route_locations(&new_obstacles, max_coord, initial_guard_posdir);
        if is_loop {
            loop_count += 1;
        }
    }
    loop_count
}

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let (obstacles, max_coord, guard_posdir) = parse_input(&input);
    let (unique_positions, _) = get_route_locations(&obstacles, max_coord, guard_posdir);
    println!(
        "P1: Number of distinct guard positions: {}",
        unique_positions.len()
    );
    println!(
        "P2: Number of possible route loops: {}",
        get_num_possible_loops(&obstacles, max_coord, guard_posdir, &unique_positions)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EX_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn ex1() {
        let (obstacles, max_coord, guard_posdir) = parse_input(EX_INPUT);
        let (unique_positions, _) = get_route_locations(&obstacles, max_coord, guard_posdir);
        assert_eq!(unique_positions.len(), 41);
    }

    #[test]
    fn ex2() {
        let (obstacles, max_coord, guard_posdir) = parse_input(EX_INPUT);
        let (unique_positions, _) = get_route_locations(&obstacles, max_coord, guard_posdir);
        assert_eq!(
            get_num_possible_loops(&obstacles, max_coord, guard_posdir, &unique_positions),
            6
        );
    }
}
