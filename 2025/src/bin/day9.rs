use itertools::Itertools;
use std::env;
use std::fs;
use std::io;
use std::iter;

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

fn area_calc(a: &Coord, b: &Coord) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

fn main() -> io::Result<()> {
    let input: Vec<_> = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
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
            area_calc(a, b)
        })
        .max()
        .unwrap();

    let last = [input[input.len() - 1], input[0]];
    let line_segs: Vec<_> = input
        .windows(2)
        .chain(iter::once(last.as_slice()))
        .collect();

    let p2_max_rect = input
        .iter()
        .combinations(2)
        .map(|ab| {
            let [a, b] = ab[..] else { unreachable!() };
            (a, b)
        })
        // sort by area in descending order so we can just take the first valid rectangle
        .sorted_by_key(|(a, b)| -(area_calc(a, b) as isize))
        .filter(|(a, b)| {
            // line segments aren't going to be largest rectangle, ignore
            a.x != b.x && a.y != b.y
        })
        .find(|(a, b)| {
            // remove any rectangles that have intersecting line segments
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
            let rect_line_segs = [[tl, tr], [tr, br], [br, bl], [bl, tl]];

            // true if no intersections
            !line_segs
                .iter()
                .cartesian_product(rect_line_segs.iter())
                .any(|(shape_ls, rect_ls)| {
                    let shape_ls_min_x = shape_ls[0].x.min(shape_ls[1].x);
                    let shape_ls_max_x = shape_ls[0].x.max(shape_ls[1].x);
                    let rect_ls_min_x = rect_ls[0].x.min(rect_ls[1].x);
                    let rect_ls_max_x = rect_ls[0].x.max(rect_ls[1].x);
                    let shape_ls_min_y = shape_ls[0].y.min(shape_ls[1].y);
                    let shape_ls_max_y = shape_ls[0].y.max(shape_ls[1].y);
                    let rect_ls_min_y = rect_ls[0].y.min(rect_ls[1].y);
                    let rect_ls_max_y = rect_ls[0].y.max(rect_ls[1].y);

                    if shape_ls_min_x == rect_ls_min_x
                        && shape_ls_max_x == rect_ls_max_x
                        && shape_ls_min_y == rect_ls_min_y
                        && shape_ls_max_y == rect_ls_max_y
                    {
                        // identical don't intersect
                        return false;
                    }

                    match (
                        shape_ls_min_x == shape_ls_max_x,
                        rect_ls_min_x == rect_ls_max_x,
                    ) {
                        // parallel lines can't intersect
                        (true, true) | (false, false) => false,
                        (true, false) => {
                            (shape_ls_min_y..=shape_ls_max_y).contains(&rect_ls_min_y)
                                // don't count end points for second check
                                && (rect_ls_min_x + 1..rect_ls_max_x).contains(&shape_ls_min_x)
                        }
                        (false, true) => {
                            (shape_ls_min_x..=shape_ls_max_x).contains(&rect_ls_min_x)
                                && (rect_ls_min_y + 1..rect_ls_max_y).contains(&shape_ls_min_y)
                        }
                    }
                })
        })
        .map(|(a, b)| area_calc(a, b))
        .unwrap();

    println!("P1: Largest rectangle area: {p1_max_rect}");
    println!("P2: Largest rectangle area: {p2_max_rect}");
    Ok(())
}
