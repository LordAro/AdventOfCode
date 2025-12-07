use memoise::memoise_map;
use std::collections::HashMap;
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
fn get_splitter_timelines(splitters: &HashSet<Coord>, y_cutoff: usize, c: Coord) -> usize {
    (if let Some(left) = get_next_splitter(splitters, y_cutoff, Coord { x: c.x - 1, y: c.y }) {
        get_splitter_timelines(splitters, y_cutoff, left)
    } else {
        1
    }) + (if let Some(right) = get_next_splitter(splitters, y_cutoff, Coord { x: c.x + 1, y: c.y })
    {
        get_splitter_timelines(splitters, y_cutoff, right)
    } else {
        1
    })
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

    let mut seen = HashSet::<Coord>::default();
    let mut splitters_hit = HashMap::<Coord, usize>::default();
    let mut to_search = vec![get_next_splitter(&splitters, last_splitter_y, start).unwrap()];
    while let Some(c) = to_search.pop() {
        if seen.contains(&c) {
            continue;
        }
        seen.insert(c);
        if c.y > last_splitter_y {
            continue;
        }

        *splitters_hit.entry(c).or_insert(0) += 1;
        if let Some(left) =
            get_next_splitter(&splitters, last_splitter_y, Coord { x: c.x - 1, y: c.y })
        {
            to_search.push(left);
        }
        if let Some(right) =
            get_next_splitter(&splitters, last_splitter_y, Coord { x: c.x + 1, y: c.y })
        {
            to_search.push(right);
        }
    }

    let splitter_count: usize = splitters_hit.keys().count();
    let num_timelines: usize = get_splitter_timelines(&splitters, last_splitter_y, start);
    println!("P1: Number of splitters hit: {splitter_count}");
    println!("P2: Number of timelines: {num_timelines}");
    Ok(())
}
