use itertools::Itertools;
use std::env;
use std::fs;
use std::io;
use std::iter;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

fn main() -> io::Result<()> {
    let input: Vec<_> = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
//    let input: Vec<_> = "7,1
//11,1
//11,7
//9,7
//9,5
//2,5
//2,3
//7,3"
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Coord {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let p1_max_rect = input
        .iter()
        .combinations(2)
        .map(|ab| {
            let [a, b] = ab[..] else { unreachable!() };
            (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
        })
        .max()
        .unwrap();

    let last = [input[input.len() - 1], input[0]];
    let line_segs: Vec<_> = input
        .windows(2)
        .chain(iter::once(last.as_slice()))
        .collect();

    let bounding_min_x = input.iter().map(|c| c.x).min().unwrap();
    let bounding_min_y = input.iter().map(|c| c.y).min().unwrap();

    let p2_max_rect = input
        .iter()
        .combinations(2)
        .map(|ab| {
            let [a, b] = ab[..] else { unreachable!() };
            (a, b)
        })
        .filter(|(a, b)| {
            // line segments aren't going to be largest rectangle, ignore
            a.x != b.x && a.y != b.y
        })
        .filter(|(a, b)| {
            // remove any rectangles that have chunks taken out of them
            !input.iter().any(|c| {
                (a.x.min(b.x) + 1..a.x.max(b.x)).contains(&c.x)
                    && (a.y.min(b.y) + 1..a.y.max(b.y)).contains(&c.y)
            })
        })
        .filter(|(a, b)| {
            // check if all 4 corners are within the shape
            let tl = Coord {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            };
            let tr = Coord {
                x: a.x.max(b.x),
                y: a.y.min(b.y),
            };
            let bl = Coord {
                x: a.x.min(b.x),
                y: a.y.max(b.y),
            };
            let br = Coord {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            };

            //println!("corner check");
            // draw line from min_bounding edge (know we're definitely outside the shape) to the corner corner
            [tl, tr, bl, br]
                .iter()
                .filter(|c| c != a && c != b) // skip our own corners
                .all(|c| {
                    //println!(" => corner: {c:?}");
                    let mut in_shape = false;
                    // start point is the minimum distance away from the bounding box
                    let distance_offset =
                        c.x.abs_diff(bounding_min_x)
                            .min(c.y.abs_diff(bounding_min_y))
                            + 1;
                    let mut search_point = Coord {
                        x: c.x - distance_offset,
                        y: c.y - distance_offset,
                    };
                    //println!("  => search start: {search_point:?}");

                    loop {
                        // use diagonal to avoid having to deal with running along the perimeter
                        search_point = Coord {
                            x: search_point.x + 1,
                            y: search_point.y + 1,
                        };
                        if in_shape && search_point == *c {
                            // don't exit again
                            break;
                        }
                        // if the search point intersects with a line segment
                        if line_segs.iter().any(|ls| {
                            let tmp = if ls[0].x == ls[1].x {
                                search_point.x == ls[0].x
                                    && search_point.y >= ls[0].y.min(ls[1].y)
                                    && search_point.y <= ls[0].y.max(ls[1].y)
                            } else {
                                search_point.y == ls[0].y
                                    && search_point.x >= ls[0].x.min(ls[1].x)
                                    && search_point.x <= ls[0].x.max(ls[1].x)
                            };
                            if tmp {
                                //println!("  => search point {search_point:?} crossed line at {ls:?}");
                            }
                            tmp
                        }) {
                            in_shape = !in_shape;
                        }
                        if search_point == *c {
                            break;
                        }
                    }
                    if !in_shape {
                        //println!(" => area: {a:?}-{b:?} found to be not be within the shape");
                    }
                    in_shape
                })
        })
        .map(|(a, b)| {
            let tmp = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            println!("{a:?} -> {b:?} = {tmp}");
            tmp
        })
        .max()
        .unwrap();

    println!("P1: Largest rectangle area: {p1_max_rect}");
    println!("P2: Largest rectangle area: {p2_max_rect}");
    Ok(())
}
