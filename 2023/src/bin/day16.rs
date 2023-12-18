use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum Mirror {
    DiagLeft,
    DiagRight,
    Vertical,
    Horizontal,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn get_next_pos(c: &Coord, dir: Dir, max_coord: &Coord) -> Option<Coord> {
    match dir {
        Dir::Up => {
            if c.y != 0 {
                return Some(Coord { x: c.x, y: c.y - 1 });
            }
        }
        Dir::Right => {
            if c.x != max_coord.x {
                return Some(Coord { x: c.x + 1, y: c.y });
            }
        }
        Dir::Down => {
            if c.y != max_coord.y {
                return Some(Coord { x: c.x, y: c.y + 1 });
            }
        }
        Dir::Left => {
            if c.x != 0 {
                return Some(Coord { x: c.x - 1, y: c.y });
            }
        }
    }
    None
}

fn get_reflections(mirror: Mirror, dir: Dir) -> Vec<Dir> {
    let mut dirs = vec![];
    match mirror {
        Mirror::DiagLeft => match dir {
            Dir::Up => dirs.push(Dir::Left),
            Dir::Right => dirs.push(Dir::Down),
            Dir::Down => dirs.push(Dir::Right),
            Dir::Left => dirs.push(Dir::Up),
        },
        Mirror::DiagRight => match dir {
            Dir::Up => dirs.push(Dir::Right),
            Dir::Right => dirs.push(Dir::Up),
            Dir::Down => dirs.push(Dir::Left),
            Dir::Left => dirs.push(Dir::Down),
        },
        Mirror::Horizontal => match dir {
            Dir::Up | Dir::Down => {
                dirs.push(Dir::Left);
                dirs.push(Dir::Right);
            }
            Dir::Left | Dir::Right => {
                dirs.push(dir);
            }
        },
        Mirror::Vertical => match dir {
            Dir::Left | Dir::Right => {
                dirs.push(Dir::Up);
                dirs.push(Dir::Down);
            }
            Dir::Up | Dir::Down => {
                dirs.push(dir);
            }
        },
    }
    dirs
}

fn run_beams(
    mirrors: &HashMap<Coord, Mirror>,
    max_coord: &Coord,
    start_posdir: (Coord, Dir),
) -> usize {
    let mut light_beams = vec![start_posdir];
    let mut energised_tiles: HashSet<(Coord, Dir)> = HashSet::new();
    energised_tiles.insert(light_beams[0]);

    while !light_beams.is_empty() {
        let mut new_light_beams = vec![];
        for (beam_pos, beam_dir) in light_beams {
            if let Some(new_pos) = get_next_pos(&beam_pos, beam_dir, max_coord) {
                if let Some(mirror) = mirrors.get(&new_pos) {
                    new_light_beams.extend(
                        get_reflections(*mirror, beam_dir)
                            .iter()
                            .map(|&d| (new_pos, d))
                            .filter(|pair| !energised_tiles.contains(pair)),
                    );
                } else {
                    // carry on in the same direction
                    if !energised_tiles.contains(&(new_pos, beam_dir)) {
                        new_light_beams.push((new_pos, beam_dir));
                    }
                }
            }
            // if off the map, no further action
        }
        energised_tiles.extend(new_light_beams.iter());
        light_beams = new_light_beams;
    }

    let unique_tiles: HashSet<Coord> = energised_tiles.iter().map(|&(c, _)| c).collect();
    unique_tiles.len()
}

fn main() -> io::Result<()> {
    let input_str: String =
        fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;

    let mirrors: HashMap<Coord, Mirror> = input_str
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c != '.')
                .map(move |(x, c)| {
                    (
                        Coord { x, y },
                        match c {
                            '\\' => Mirror::DiagLeft,
                            '/' => Mirror::DiagRight,
                            '|' => Mirror::Vertical,
                            '-' => Mirror::Horizontal,
                            _ => unreachable!(),
                        },
                    )
                })
        })
        .collect();

    // Kinda cheating as it assumes there's at least one mirror on the outer edges
    let max_coord = Coord {
        x: mirrors.keys().max_by_key(|m| m.x).unwrap().x,
        y: mirrors.keys().max_by_key(|m| m.y).unwrap().y,
    };

    println!(
        "Total energised tiles: {}",
        run_beams(&mirrors, &max_coord, (Coord { x: 0, y: 0 }, Dir::Right))
    );

    let maximum_energised_tiles: usize = [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
        .iter()
        .flat_map(|&d| {
            let r: Vec<_> = match d {
                Dir::Up | Dir::Down => (0..=max_coord.x)
                    .map(|x| {
                        (
                            Coord {
                                x,
                                y: if d == Dir::Up { max_coord.y } else { 0 },
                            },
                            d,
                        )
                    })
                    .collect(),
                Dir::Left | Dir::Right => (0..=max_coord.y)
                    .map(|y| {
                        (
                            Coord {
                                x: if d == Dir::Left { max_coord.x } else { 0 },
                                y,
                            },
                            d,
                        )
                    })
                    .collect(),
            };
            r.iter()
                .map(|&pd| run_beams(&mirrors, &max_coord, pd))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap();

    println!(
        "Maximum number of energised tiles: {}",
        maximum_energised_tiles
    );

    Ok(())
}
