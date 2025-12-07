use memoise::memoise_map;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

fn get_next_splitter(splitters: &HashSet<Coord>, y_cutoff: usize, c: Coord) -> Option<Coord> {
    for y in c.y + 1..=y_cutoff {
        if splitters.contains(&Coord { x: c.x, y }) {
            return Some(Coord { x: c.x, y });
        }
    }
    None
}

#[memoise_map(c)]
fn get_splitter_timelines(
    splitters: &HashSet<Coord>,
    seen_splitters: &mut HashSet<Coord>,
    y_cutoff: usize,
    c: Coord,
) -> usize {
    seen_splitters.insert(c);
    let next_splitter_left = get_next_splitter(splitters, y_cutoff, Coord { x: c.x - 1, y: c.y });
    let next_splitter_right = get_next_splitter(splitters, y_cutoff, Coord { x: c.x + 1, y: c.y });
    let left_total = next_splitter_left
        .map(|s| get_splitter_timelines(splitters, seen_splitters, y_cutoff, s))
        .unwrap_or(1);
    let right_total = next_splitter_right
        .map(|s| get_splitter_timelines(splitters, seen_splitters, y_cutoff, s))
        .unwrap_or(1);
    left_total + right_total
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;
    let start = Coord {
        x: input.lines().next().unwrap().find('S').unwrap(),
        y: 0,
    };
    let splitters: HashSet<Coord> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, cell)| (cell == '^').then_some(Coord { x, y }))
        })
        .collect();

    let last_splitter_y = splitters.iter().max_by_key(|c| c.y).unwrap().y;

    let mut seen_splitters = HashSet::<Coord>::default();
    let num_timelines: usize =
        get_splitter_timelines(&splitters, &mut seen_splitters, last_splitter_y, start);
    let splitter_count: usize = seen_splitters.len();
    println!("P1: Number of splitters hit: {splitter_count}");
    println!("P2: Number of timelines: {num_timelines}");
    Ok(())
}
