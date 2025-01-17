use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Rock {
    c: Coord,
    is_cube: bool,
}

fn move_rocks_north(rocks: &mut [Rock]) {
    rocks.sort_by(|a, b| a.c.x.cmp(&b.c.x).then(a.c.y.cmp(&b.c.y)));

    for (_, group) in &rocks.iter_mut().chunk_by(|r| r.c.x) {
        let mut previous: Option<&Rock> = None;
        for r in group {
            if !r.is_cube {
                r.c.y = previous.map(|p| p.c.y + 1).unwrap_or(0);
            }
            previous = Some(r);
        }
    }
}

fn move_rocks_south(rocks: &mut [Rock], max_y: usize) {
    rocks.sort_by(|a, b| a.c.x.cmp(&b.c.x).then(b.c.y.cmp(&a.c.y)));

    for (_, group) in &rocks.iter_mut().chunk_by(|r| r.c.x) {
        let mut previous: Option<&Rock> = None;
        for r in group {
            if !r.is_cube {
                r.c.y = previous.map(|p| p.c.y - 1).unwrap_or(max_y);
            }
            previous = Some(r);
        }
    }
}

fn move_rocks_west(rocks: &mut [Rock]) {
    rocks.sort_by(|a, b| a.c.y.cmp(&b.c.y).then(a.c.x.cmp(&b.c.x)));

    for (_, group) in &rocks.iter_mut().chunk_by(|r| r.c.y) {
        let mut previous: Option<&Rock> = None;
        for r in group {
            if !r.is_cube {
                r.c.x = previous.map(|p| p.c.x + 1).unwrap_or(0);
            }
            previous = Some(r);
        }
    }
}

fn move_rocks_east(rocks: &mut [Rock], max_x: usize) {
    rocks.sort_by(|a, b| a.c.y.cmp(&b.c.y).then(b.c.x.cmp(&a.c.x)));

    for (_, group) in &rocks.iter_mut().chunk_by(|r| r.c.y) {
        let mut previous: Option<&Rock> = None;
        for r in group {
            if !r.is_cube {
                r.c.x = previous.map(|p| p.c.x - 1).unwrap_or(max_x);
            }
            previous = Some(r);
        }
    }
}

fn move_rocks_cycle(rocks: &mut [Rock], max_coord: &Coord) {
    move_rocks_north(rocks);
    move_rocks_west(rocks);
    move_rocks_south(rocks, max_coord.y);
    move_rocks_east(rocks, max_coord.x);
}

fn calc_north_load(rocks: &[Rock], max_coord: &Coord) -> usize {
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

    let mut north_rocks = rocks.clone();
    move_rocks_north(&mut north_rocks);
    println!("Total load: {}", calc_north_load(&north_rocks, &max_coord));

    let mut cycle_cache: HashMap<Vec<Rock>, usize> = HashMap::new();
    cycle_cache.insert(rocks.clone(), 0);

    let mut cycle_start = 0;
    let mut cycle_length = 0;
    let mut cur_rocks = rocks;
    // look for cycle in cycles
    for cycle_number in 1..1_000_000_000 {
        move_rocks_cycle(&mut cur_rocks, &max_coord);
        if let Some(idx) = cycle_cache.get(&cur_rocks) {
            cycle_start = *idx;
            cycle_length = cycle_number - cycle_start;
            break;
        }
        cycle_cache.insert(cur_rocks.clone(), cycle_number);
    }

    let remaining_cycles = 1_000_000_000 - (cycle_start + cycle_length);
    let offset = remaining_cycles % cycle_length;

    // Don't have a good way of finding cycle_start + offset in the hashmap,
    // so just run offset more cycles to get to where we want to be
    for _ in 0..offset {
        move_rocks_cycle(&mut cur_rocks, &max_coord);
    }

    println!(
        "Total load after N cycles: {}",
        calc_north_load(&cur_rocks, &max_coord)
    );

    Ok(())
}
