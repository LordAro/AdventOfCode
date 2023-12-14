use std::env;
use std::fs;
use std::io;

extern crate itertools;
use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Rock {
    c: Coord,
    is_cube: bool,
}

fn move_rocks_north(rocks: &[Rock]) -> Vec<Rock> {
    let sorted_rocks = rocks
        .iter()
        .sorted_by(|a, b| a.c.y.cmp(&b.c.y).then(a.c.x.cmp(&b.c.y)))
        .collect_vec();

    let mut new_rocks: Vec<Rock> = vec![];
    for r in sorted_rocks {
        // cubes don't move
        if r.is_cube {
            new_rocks.push(r.clone());
        } else if let Some(blocking_rock) = new_rocks
            .iter()
            .filter(|&o| o.c.x == r.c.x)
            .max_by_key(|&o| o.c.y)
        {
            new_rocks.push(Rock {
                c: Coord {
                    x: r.c.x,
                    y: blocking_rock.c.y + 1,
                },
                is_cube: false,
            });
        } else {
            new_rocks.push(Rock {
                c: Coord { x: r.c.x, y: 0 },
                is_cube: false,
            });
        }
    }
    new_rocks
}

fn move_rocks_south(rocks: &[Rock]) -> Vec<Rock> {
    let max_y = rocks.iter().max_by_key(|r| r.c.y).unwrap().c.y;
    let sorted_rocks = rocks
        .iter()
        .sorted_by(|a, b| b.c.y.cmp(&a.c.y).then(a.c.x.cmp(&b.c.y)))
        .collect_vec();

    let mut new_rocks: Vec<Rock> = vec![];
    for r in sorted_rocks {
        // cubes don't move
        if r.is_cube {
            new_rocks.push(r.clone());
        } else if let Some(blocking_rock) = new_rocks
            .iter()
            .filter(|&o| o.c.x == r.c.x)
            .min_by_key(|&o| o.c.y)
        {
            new_rocks.push(Rock {
                c: Coord {
                    x: r.c.x,
                    y: blocking_rock.c.y - 1,
                },
                is_cube: false,
            });
        } else {
            new_rocks.push(Rock {
                c: Coord { x: r.c.x, y: max_y },
                is_cube: false,
            });
        }
    }
    new_rocks
}

fn move_rocks_west(rocks: &[Rock]) -> Vec<Rock> {
    let sorted_rocks = rocks
        .iter()
        .sorted_by(|a, b| a.c.x.cmp(&b.c.x).then(a.c.y.cmp(&b.c.y)))
        .collect_vec();

    let mut new_rocks: Vec<Rock> = vec![];
    for r in sorted_rocks {
        // cubes don't move
        if r.is_cube {
            new_rocks.push(r.clone());
        } else if let Some(blocking_rock) = new_rocks
            .iter()
            .filter(|&o| o.c.y == r.c.y)
            .max_by_key(|&o| o.c.x)
        {
            new_rocks.push(Rock {
                c: Coord {
                    x: blocking_rock.c.x + 1,
                    y: r.c.y,
                },
                is_cube: false,
            });
        } else {
            new_rocks.push(Rock {
                c: Coord { x: 0, y: r.c.y },
                is_cube: false,
            });
        }
    }
    new_rocks
}

fn move_rocks_east(rocks: &[Rock]) -> Vec<Rock> {
    let max_x = rocks.iter().max_by_key(|r| r.c.x).unwrap().c.x;
    let sorted_rocks = rocks
        .iter()
        .sorted_by(|a, b| b.c.x.cmp(&a.c.x).then(a.c.y.cmp(&b.c.y)))
        .collect_vec();

    let mut new_rocks: Vec<Rock> = vec![];
    for r in sorted_rocks {
        // cubes don't move
        if r.is_cube {
            new_rocks.push(r.clone());
        } else if let Some(blocking_rock) = new_rocks
            .iter()
            .filter(|&o| o.c.y == r.c.y)
            .min_by_key(|&o| o.c.x)
        {
            new_rocks.push(Rock {
                c: Coord {
                    x: blocking_rock.c.x - 1,
                    y: r.c.y,
                },
                is_cube: false,
            });
        } else {
            new_rocks.push(Rock {
                c: Coord { x: max_x, y: r.c.y },
                is_cube: false,
            });
        }
    }
    new_rocks
}

fn move_rocks_cycle(rocks: &[Rock]) -> Vec<Rock> {
    move_rocks_east(&move_rocks_south(&move_rocks_west(&move_rocks_north(
        rocks,
    ))))
}

fn calc_north_load(rocks: &[Rock], max_coord: Coord) -> usize {
    rocks
        .iter()
        .filter(|&r| !r.is_cube)
        .map(|r| max_coord.y + 1 - r.c.y)
        .sum()
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;
    let rocks: Vec<_> = input_str
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#' || c == 'O')
                .map(move |(x, c)| match c {
                    '#' => Rock {
                        c: Coord { x, y },
                        is_cube: true,
                    },
                    'O' => Rock {
                        c: Coord { x, y },
                        is_cube: false,
                    },
                    _ => unreachable!(),
                })
        })
        .collect();

    let max_coord = Coord {
        x: rocks.iter().max_by_key(|r| r.c.x).unwrap().c.x,
        y: rocks.iter().max_by_key(|r| r.c.y).unwrap().c.y,
    };

    println!(
        "Total load: {}",
        calc_north_load(&move_rocks_north(&rocks), max_coord)
    );

    let mut cache_cycle: Vec<Vec<Rock>> = vec![];
    cache_cycle.push(rocks.clone());

    let mut cycle_start = 0;
    let mut cycle_length = 0;
    let mut cur_rocks = rocks;
    // look for cycle in cycles
    for cycle_number in 1..1_000_000_000 {
        cur_rocks = move_rocks_cycle(&cur_rocks);
        if let Some(idx) = cache_cycle.iter().position(|r| r == &cur_rocks) {
            cycle_start = idx;
            cycle_length = cycle_number - cycle_start;
            break;
        }
        cache_cycle.push(cur_rocks.clone());
    }

    let remaining_cycles = 1_000_000_000 - (cycle_start + cycle_length);
    let offset = remaining_cycles % cycle_length;

    println!(
        "Total load after N cycles: {}",
        calc_north_load(&cache_cycle[cycle_start + offset], max_coord)
    );

    Ok(())
}
