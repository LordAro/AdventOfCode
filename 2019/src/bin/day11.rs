use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate advent_of_code;
use advent_of_code::intcode;

fn paint(program: &[isize], start_tile: isize) -> HashMap<(isize, isize), isize> {
    let mut painted_tiles: HashMap<(isize, isize), isize> = HashMap::new();
    let mut cur_pos = (0, 0);
    let mut cur_dir = 0;
    let mut mach = intcode::Machine::new(program, &[start_tile]);
    while let Some(colour) = mach.run_until_output() {
        *painted_tiles.entry(cur_pos).or_insert(0) = colour;
        let turn_dir = mach.run_until_output().unwrap();
        cur_dir = ((cur_dir + if turn_dir == 1 { 1 } else { -1 }) + 4) % 4;
        cur_pos = match cur_dir {
            0 => (cur_pos.0, cur_pos.1 - 1), // up (negative y)
            1 => (cur_pos.0 + 1, cur_pos.1), // right
            2 => (cur_pos.0, cur_pos.1 + 1), // down (positive y)
            3 => (cur_pos.0 - 1, cur_pos.1), // left
            _ => panic!("Unrecognised current direction {}", cur_dir),
        };
        mach.push_input(*painted_tiles.get(&cur_pos).unwrap_or(&0));
    }
    return painted_tiles;
}

fn main() {
    let program_str = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .next()
    .unwrap()
    .unwrap();

    let program = intcode::read_input(&program_str);

    println!(
        "Number of painted tiles: {}",
        paint(&program, 0).keys().count()
    );

    let tile_colours = paint(&program, 1);
    println!("Registration identifier:");
    let mut tiles: Vec<_> = tile_colours.keys().collect();
    tiles.sort_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    let min_coord = tiles.first().unwrap();
    let max_coord = tiles.last().unwrap();
    for j in min_coord.1..=max_coord.1 {
        for i in min_coord.0..=max_coord.0 {
            print!(
                "{}",
                if tile_colours.get(&(i, j)).unwrap_or(&0) == &0 {
                    ' '
                } else {
                    '#'
                }
            );
        }
        println!();
    }
}
