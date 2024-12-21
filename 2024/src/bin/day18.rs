use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

fn coord_neighbours(c: Coord) -> impl Iterator<Item = Coord> {
    [
        c.y.checked_sub(1).map(|y| Coord { x: c.x, y }),
        (c.y < 70).then_some(Coord { x: c.x, y: c.y + 1 }),
        c.x.checked_sub(1).map(|x| Coord { x, y: c.y }),
        (c.x < 70).then_some(Coord { x: c.x + 1, y: c.y }),
    ]
    .into_iter()
    .flatten()
}

fn get_path_len(corrupted_bytes: &HashSet<Coord>) -> Option<usize> {
    let mut to_search = BTreeSet::from([(0, Coord { x: 0, y: 0 })]);
    let mut searched: HashSet<Coord> = HashSet::default();
    while let Some((node_dist, node)) = to_search.pop_first() {
        if node == (Coord { x: 70, y: 70 }) {
            return Some(node_dist);
        }

        searched.insert(node);

        for n in
            coord_neighbours(node).filter(|n| !searched.contains(n) && !corrupted_bytes.contains(n))
        {
            to_search.insert((node_dist + 1, n));
        }
    }

    None
}

fn main() -> io::Result<()> {
    let corrupted_bytes: Vec<_> =
        fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(",").unwrap();
                Coord {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect();

    let mut corrupted_bytes_1kb: HashSet<_> = corrupted_bytes.iter().take(1024).copied().collect();

    let path_len = get_path_len(&corrupted_bytes_1kb).unwrap();
    println!("P1: Minimum number of steps required: {path_len}");

    let mut blocking_byte = None;
    for byte in &corrupted_bytes[1024..] {
        corrupted_bytes_1kb.insert(*byte);
        if get_path_len(&corrupted_bytes_1kb).is_none() {
            blocking_byte = Some(byte);
            break;
        }
    }
    let blocking_byte = blocking_byte.unwrap();
    println!(
        "P2: First coord to block exit: {},{}",
        blocking_byte.x, blocking_byte.y
    );

    Ok(())
}
