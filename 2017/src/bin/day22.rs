use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Dir {
    North,
    East,
    South,
    West,
}

fn turn_left(d: Dir) -> Dir {
    match d {
        Dir::North => Dir::West,
        Dir::East => Dir::North,
        Dir::South => Dir::East,
        Dir::West => Dir::South,
    }
}

fn turn_right(d: Dir) -> Dir {
    match d {
        Dir::North => Dir::East,
        Dir::East => Dir::South,
        Dir::South => Dir::West,
        Dir::West => Dir::North,
    }
}

fn turn_rev(d: Dir) -> Dir {
    match d {
        Dir::North => Dir::South,
        Dir::East => Dir::West,
        Dir::South => Dir::North,
        Dir::West => Dir::East,
    }
}

fn move_pos(pos: (isize, isize), d: &Dir) -> (isize, isize) {
    match *d {
        Dir::North => (pos.0, pos.1 + 1),
        Dir::East => (pos.0 + 1, pos.1),
        Dir::South => (pos.0, pos.1 - 1),
        Dir::West => (pos.0 - 1, pos.1),
    }
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input: Vec<Vec<_>> = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    // Initial setup
    let middle = ((input.len() as isize + 1) / 2) - 1;
    let mut grid = HashMap::new();
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            grid.insert((x as isize - middle, middle - y as isize), *cell);
        }
    }
    let mut grid2 = grid.clone();

    let mut infected = 0;
    let mut pos = (0, 0);
    let mut dir = Dir::North;
    for _ in 0..10000 {
        let val = grid.entry(pos).or_insert('.');
        dir = if *val == '#' {
            turn_right(dir)
        } else {
            turn_left(dir)
        };

        *val = if *val == '.' { '#' } else { '.' };
        if *val == '#' {
            infected += 1;
        }

        pos = move_pos(pos, &dir);
    }

    println!("Infected count: {}", infected);

    infected = 0;
    pos = (0, 0);
    dir = Dir::North;
    for _ in 0..10_000_000 {
        let val = grid2.entry(pos).or_insert('.');
        dir = match *val {
            '.' => turn_left(dir),
            'W' => dir,
            '#' => turn_right(dir),
            'F' => turn_rev(dir),
            _ => unreachable!(),
        };

        *val = match *val {
            '.' => 'W',
            'W' => '#',
            '#' => 'F',
            'F' => '.',
            _ => unreachable!(),
        };
        if *val == '#' {
            infected += 1;
        }

        pos = move_pos(pos, &dir);
    }
    println!("Evolved infected: {}", infected);
}
