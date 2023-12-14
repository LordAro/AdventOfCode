use std::env;
use std::fs;
use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Rock {
    c: Coord,
    is_cube: bool,
}

fn move_rocks_north(rocks: &[Rock]) -> Vec<Rock> {
    // rocks are already in correct order for moving north
    let mut new_rocks: Vec<Rock> = vec![];
    for r in rocks {
        // cubes don't move
        if r.is_cube {
            new_rocks.push(r.clone());
        } else {
            if let Some(blocking_rock) = new_rocks
                .iter()
                .filter(|&o| o.c.x == r.c.x)
                .max_by_key(|&o| o.c.y)
            {
                new_rocks.push(Rock {
                    c: Coord {
                        x: r.c.x,
                        y: blocking_rock.c.y + 1,
                    },
                    is_cube: false,
                });
            } else {
                new_rocks.push(Rock {
                    c: Coord { x: r.c.x, y: 0 },
                    is_cube: false,
                });
            }
        }
    }
    new_rocks
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;
    //    let input_str = "O....#....
    //O.OO#....#
    //.....##...
    //OO.#O....O
    //.O.....O#.
    //O.#..O.#.#
    //..O..#O..O
    //.......O..
    //#....###..
    //#OO..#....
    //";
    let rocks: Vec<_> = input_str
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#' || c == 'O')
                .map(move |(x, c)| match c {
                    '#' => Rock {
                        c: Coord { x, y },
                        is_cube: true,
                    },
                    'O' => Rock {
                        c: Coord { x, y },
                        is_cube: false,
                    },
                    _ => unreachable!(),
                })
        })
        .collect();

    let max_y = rocks.iter().max_by_key(|r| r.c.y).unwrap().c.y;

    let new_rocks = move_rocks_north(&rocks);
    let total_north_load: usize = new_rocks
        .iter()
        .filter(|&r| !r.is_cube)
        .map(|r| max_y + 1 - r.c.y)
        .sum();

    println!("Total load: {}", total_north_load);

    Ok(())
}
