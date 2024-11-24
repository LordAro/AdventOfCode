use std::fs;
use std::io;

struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone)]
enum State {
    None,
    Silver(usize),
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<State>>) {
    for row in grid {
        for cell in row {
            match cell {
                State::None => print!("."),
                State::Silver(d) => {
                    if *d == 0 {
                        print!("#");
                    } else {
                        print!("{d:x}");
                    }
                }
            };
        }
        println!();
    }
}

fn can_be_dug<const INCLUDE_DIAG: bool>(grid: &[Vec<State>], c: &Coord) -> bool {
    let neighbours = if INCLUDE_DIAG {
        vec![
            // NW
            (c.x > 0 && c.y > 0).then_some(Coord {
                x: c.x - 1,
                y: c.y - 1,
            }),
            // N
            (c.y > 0).then_some(Coord { x: c.x, y: c.y - 1 }),
            // NE
            (c.y > 0 && c.x < grid[c.y].len() - 1).then_some(Coord {
                x: c.x + 1,
                y: c.y - 1,
            }),
            // W
            (c.x > 0).then_some(Coord { x: c.x - 1, y: c.y }),
            // E
            (c.x < grid[c.y].len() - 1).then_some(Coord { x: c.x + 1, y: c.y }),
            // SW
            (c.y < grid.len() - 1 && c.x > 0).then_some(Coord {
                x: c.x - 1,
                y: c.y + 1,
            }),
            // S
            (c.y < grid.len() - 1).then_some(Coord { x: c.x, y: c.y + 1 }),
            // SE
            (c.y < grid.len() - 1 && c.x < grid[c.y].len() - 1).then_some(Coord {
                x: c.x + 1,
                y: c.y + 1,
            }),
        ]
    } else {
        vec![
            c.y.checked_sub(1).map(|y| Coord { x: c.x, y }),
            Some(c.y + 1)
                .filter(|y| *y < grid.len())
                .map(|y| Coord { x: c.x, y }),
            c.x.checked_sub(1).map(|x| Coord { x, y: c.y }),
            Some(c.x + 1)
                .filter(|x| *x < grid[c.y].len())
                .map(|x| Coord { x, y: c.y }),
        ]
    };

    let State::Silver(cur_depth) = grid[c.y][c.x] else {
        unreachable!()
    };

    neighbours.iter().all(|maybe_n| match maybe_n {
        None => cur_depth == 0,
        Some(n) => match grid[n.y][n.x] {
            State::None => cur_depth == 0,
            State::Silver(d) => cur_depth == d,
        },
    })
}

fn dig_grid_step<const INCLUDE_DIAG: bool>(grid: &[Vec<State>]) -> (Vec<Vec<State>>, usize) {
    let mut new_grid = grid.to_owned();
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = &grid[y][x];
            if matches!(c, State::Silver(_)) && can_be_dug::<INCLUDE_DIAG>(grid, &Coord { x, y }) {
                match c {
                    State::Silver(d) => {
                        new_grid[y][x] = State::Silver(d + 1);
                        count += 1;
                    }
                    State::None => unreachable!(),
                }
            }
        }
    }
    (new_grid, count)
}

fn parse_grid(input_str: &str) -> Vec<Vec<State>> {
    input_str
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => State::None,
                    '#' => State::Silver(0),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn get_dig_total<const INCLUDE_DIAG: bool>(input_str: &str) -> usize {
    let mut grid = parse_grid(input_str);
    let mut total_removed_earth = 0;
    loop {
        let (new_grid, total_removed) = dig_grid_step::<INCLUDE_DIAG>(&grid);
        grid = new_grid;
        total_removed_earth += total_removed;
        if total_removed == 0 {
            break;
        }
    }
    //print_grid(&grid);
    total_removed_earth
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_total_removed_earth = get_dig_total::<false>(&fs::read_to_string(p1_input_filename)?);
    println!("P1: Total removed earth blocks: {p1_total_removed_earth}");

    let p2_total_removed_earth = get_dig_total::<false>(&fs::read_to_string(p2_input_filename)?);
    println!("P2: Total removed earth blocks: {p2_total_removed_earth}");

    let p3_total_removed_earth = get_dig_total::<true>(&fs::read_to_string(p3_input_filename)?);
    println!("P3: Total removed earth blocks: {p3_total_removed_earth}");

    Ok(())
}
