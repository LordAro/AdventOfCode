use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

fn parse_guard(l: &str) -> (Coord, Coord) {
    let (p, v) = l.split_once(' ').unwrap();
    let (p, v) = (
        p[2..].split_once(',').unwrap(),
        v[2..].split_once(',').unwrap(),
    );
    let (p, v) = (
        Coord {
            x: p.0.parse().unwrap(),
            y: p.1.parse().unwrap(),
        },
        Coord {
            x: v.0.parse().unwrap(),
            y: v.1.parse().unwrap(),
        },
    );
    (p, v)
}

fn get_guard_position<const GRID_X: i64, const GRID_Y: i64>(
    guard: (Coord, Coord),
    seconds: i64,
) -> Coord {
    Coord {
        x: (guard.0.x + seconds * guard.1.x).rem_euclid(GRID_X),
        y: (guard.0.y + seconds * guard.1.y).rem_euclid(GRID_Y),
    }
}

fn count_quadrants<const GRID_X: i64, const GRID_Y: i64>(guards: &[Coord]) -> Vec<usize> {
    // TODO: there's a better way of doing this
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for g in guards {
        if g.x < GRID_X / 2 && g.y < GRID_Y / 2 {
            q1 += 1;
        } else if g.x > GRID_X / 2 && g.y < GRID_Y / 2 {
            q2 += 1;
        } else if g.x < GRID_X / 2 && g.y > GRID_Y / 2 {
            q3 += 1;
        } else if g.x > GRID_X / 2 && g.y > GRID_Y / 2 {
            q4 += 1;
        }
    }
    vec![q1, q2, q3, q4]
}

#[allow(dead_code)] // debugging
fn print_guards<const GRID_X: i64, const GRID_Y: i64>(guards: &HashSet<Coord>) {
    for y in 0..GRID_Y {
        for x in 0..GRID_X {
            if guards.contains(&Coord { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    for _ in 0..GRID_X {
        print!("-");
    }
    println!();
}

fn get_christmas_tree<const GRID_X: i64, const GRID_Y: i64>(guards: &[(Coord, Coord)]) -> i64 {
    // Very much underspecified problem :(
    // Look for filled in section of the top of the tree
    for s in 0.. {
        let guards_after_move: HashSet<_> = guards
            .iter()
            .map(|g| get_guard_position::<GRID_X, GRID_Y>(*g, s))
            .collect();
        for g in &guards_after_move {
            if (0..5)
                .flat_map(|y| {
                    (-y..=y).map(move |x| Coord {
                        x: g.x + x,
                        y: g.y + y,
                    })
                })
                .all(|c| guards_after_move.contains(&c))
            {
                //print_guards::<GRID_X, GRID_Y>(&guards_after_move);
                return s;
            }
        }
    }
    unreachable!()
}

fn main() -> io::Result<()> {
    let guards: Vec<_> = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
        .lines()
        .map(parse_guard)
        .collect();

    const GRID_X: i64 = 101;
    const GRID_Y: i64 = 103;

    let guards_after_move: Vec<_> = guards
        .iter()
        .map(|g| get_guard_position::<GRID_X, GRID_Y>(*g, 100))
        .collect();

    let safety_factor: usize = count_quadrants::<GRID_X, GRID_Y>(&guards_after_move)
        .iter()
        .product();

    println!("P1: Safety factor after 100s: {safety_factor}");

    let time = get_christmas_tree::<GRID_X, GRID_Y>(&guards);
    println!("P2: Christmas tree appears after: {time} seconds");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let guards: Vec<_> = input.lines().map(parse_guard).collect();
        let guards_after_move: Vec<_> = guards
            .iter()
            .map(|g| get_guard_position::<11, 7>(*g, 100))
            .collect();
        let safety_factor: usize = count_quadrants::<11, 7>(&guards_after_move)
            .iter()
            .product();
        assert_eq!(safety_factor, 12);
    }
}
