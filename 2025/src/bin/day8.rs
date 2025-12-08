use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

fn line_dist(a: &Coord, b: &Coord) -> isize {
    let c = Coord {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    };
    (c.x * c.x + c.y * c.y + c.z * c.z).isqrt()
}

fn main() -> io::Result<()> {
    let input: Vec<_> = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
        .lines()
        .map(|l| {
            let mut sp = l.split(',');
            Coord {
                x: sp.next().unwrap().parse().unwrap(),
                y: sp.next().unwrap().parse().unwrap(),
                z: sp.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut p1_circuit_prod: usize = 0;
    let mut p2_last_pair_x: isize = 0;
    let mut circuits: Vec<HashSet<&Coord>> = vec![];
    for (n, ab) in input
        .iter()
        .combinations(2)
        .sorted_by(|ap, bp| line_dist(ap[0], ap[1]).cmp(&line_dist(bp[0], bp[1])))
        .enumerate()
    {
        let c0 = circuits.iter().position(|c| c.contains(ab[0]));
        let c1 = circuits.iter().position(|c| c.contains(ab[1]));
        match (c0, c1) {
            (Some(x), Some(y)) if x == y => {}
            (Some(x), Some(y)) => {
                // merge circuits
                let x1 = x.min(y);
                let y1 = x.max(y);
                let mut orig = circuits.remove(y1);
                circuits[x1].extend(orig.drain());
            }
            (Some(x), None) | (None, Some(x)) => {
                circuits[x].insert(ab[0]);
                circuits[x].insert(ab[1]);
            }
            (None, None) => {
                let new_circuit = ab.clone().into_iter().collect();
                circuits.push(new_circuit);
            }
        }
        if n == 1000 {
            p1_circuit_prod = circuits
                .iter()
                .k_largest_by_key(3, |c| c.len())
                .map(|c| c.len())
                .product();
        }
        if circuits.len() == 1 && circuits[0].len() == 1000 {
            p2_last_pair_x = ab[0].x * ab[1].x;
            break;
        }
    }

    println!("P1: Product of 3 largest circuits: {p1_circuit_prod}");
    println!("P2: Product of X-coord of last 2 junction boxes to be connected: {p2_last_pair_x}");
    Ok(())
}
