use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

fn count_neighbours(rolls: &HashSet<Coord>, c: Coord) -> usize {
    [
        // -y
        c.y.checked_sub(1)
            .zip(c.x.checked_sub(1))
            .map(|(y, x)| Coord { x, y }),
        c.y.checked_sub(1).map(|y| Coord { x: c.x, y }),
        c.y.checked_sub(1).map(|y| Coord { x: c.x + 1, y }),
        // y
        c.x.checked_sub(1).map(|x| Coord { x, y: c.y }),
        Some(Coord { x: c.x + 1, y: c.y }),
        // +y
        c.x.checked_sub(1).map(|x| Coord { x, y: c.y + 1 }),
        Some(Coord { x: c.x, y: c.y + 1 }),
        Some(Coord {
            x: c.x + 1,
            y: c.y + 1,
        }),
    ]
    .into_iter()
    .flatten()
    .filter(|c| rolls.contains(&c))
    .count()
}

fn main() -> io::Result<()> {
    let mut input: HashSet<Coord> =
        fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| (c == '@').then_some(Coord { x, y }))
            })
            .collect();

    let p1_movable_rolls = input
        .iter()
        .filter(|c| count_neighbours(&input, **c) < 4)
        .count();

    let mut p2_movable_rolls = 0;
    let mut newly_removed_rolls = 1;
    while newly_removed_rolls > 0 {
        let input_cpy = input.clone();
        newly_removed_rolls = input
            .extract_if(|c| count_neighbours(&input_cpy, *c) < 4)
            .count();
        p2_movable_rolls += newly_removed_rolls;
    }

    println!("P1: Number of movable paper rolls: {p1_movable_rolls}");
    println!("P1: Total number of movable paper rolls: {p2_movable_rolls}");
    Ok(())
}
