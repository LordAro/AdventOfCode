use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_wall(x: usize, y: usize, magic: usize) -> bool {
    // Odd number of set bits == wall
    (x * x + 3 * x + 2 * x * y + y + y * y + magic).count_ones() % 2 != 0
}

fn find_route(start: (usize, usize), magic: usize) -> (usize, usize) {
    let mut new_poses = vec![start];
    let mut seen_points = HashSet::new();
    seen_points.insert(start);

    // Hacky stuff to get the answers out
    let mut p1depth = 0;
    let mut p2points = 0;

    let mut depth = 0;
    while new_poses.len() > 0 && p1depth == 0 {
        let mut queue = vec![];
        for pos in new_poses.drain(..) {
            let mut next_poses = vec![(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)];
            if pos.0 > 0 {
                next_poses.push((pos.0 - 1, pos.1));
            }
            if pos.1 > 0 {
                next_poses.push((pos.0, pos.1 - 1));
            }
            for next in &next_poses {
                if !is_wall(next.0, next.1, magic) && !seen_points.contains(next) {
                    queue.push(*next);
                    seen_points.insert(*next);
                }
            }
        }
        depth += 1;

        // Find goal states
        if depth == 50 {
            // p2
            p2points = seen_points.len();
        }
        for q in &queue {
            if q.0 == 31 && q.1 == 39 {
                // p1
                p1depth = depth;
            }
        }
        new_poses = queue;
    }
    return (p1depth, p2points);
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();

    let start = (1, 1);
    let (p1depth, p2points) = find_route(start, input);
    println!("Route length: {}", p1depth);
    println!("Number of points visited: {}", p2points);
}
