#[macro_use]
extern crate itertools;

use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pattern = Vec<Vec<char>>;

fn rot2_ccw(input: &Pattern) -> Pattern {
    izip!(&input[0], &input[1])
        .rev()
        .map(|(&a, &b)| vec![a, b])
        .collect()
}

fn rot3_ccw(input: &Pattern) -> Pattern {
    izip!(&input[0], &input[1], &input[2])
        .rev()
        .map(|(&a, &b, &c)| vec![a, b, c])
        .collect()
}

fn light_total(input: &Pattern) -> usize {
    input
        .iter()
        .flat_map(|v| v.iter())
        .filter(|&&c| c == '#')
        .count()
}

fn flip_v(input: &Pattern) -> Pattern {
    input
        .clone()
        .into_iter()
        .map(|v| v.into_iter().rev().collect())
        .collect()
}

fn flip_h(input: &Pattern) -> Pattern {
    input.clone().into_iter().rev().collect()
}

fn variants(input: &Pattern) -> Vec<Pattern> {
    let base = input.clone();
    let rot90 = if input.len() == 2 {
        rot2_ccw(&rot2_ccw(&rot2_ccw(&input)))
    } else {
        rot3_ccw(&rot3_ccw(&rot3_ccw(&input)))
    };
    let rot180 = if input.len() == 2 {
        rot2_ccw(&rot2_ccw(&input))
    } else {
        rot3_ccw(&rot3_ccw(&input))
    };
    let rot270 = if input.len() == 2 {
        rot2_ccw(&input)
    } else {
        rot3_ccw(&input)
    };
    let rots = [base, rot90, rot180, rot270];
    itertools::Itertools::flatten(
        rots.into_iter()
            .map(|&ref r| vec![r.clone(), flip_h(r), flip_v(&r), flip_v(&flip_h(&r))]),
    )
    .unique()
    .collect()
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input: Vec<(_, _)> = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let kv: Vec<_> = line.split(" => ").collect();
            let k: Pattern = kv[0].split('/').map(|w| w.chars().collect()).collect();
            let v: Pattern = kv[1].split('/').map(|w| w.chars().collect()).collect();
            (k, v)
        })
        .collect();

    let mut grid = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    for n in 0..18 {
        let g_size = if grid.len() % 2 == 0 { 2 } else { 3 };
        let len = grid.len() / g_size;

        let mut newgrid = if g_size == 2 {
            vec![vec!['.'; len * 3]; len * 3]
        } else {
            vec![vec!['.'; len * 4]; len * 4]
        };
        for i in 0..len {
            for j in 0..len {
                let subgrid: Pattern = grid
                    .iter()
                    .skip(i * g_size)
                    .take(g_size)
                    .map(|v| v.iter().skip(j * g_size).take(g_size).cloned().collect())
                    .collect();

                // find matching pattern
                let new_val = input
                    .iter()
                    .filter(|&&(ref k, _)| {
                        k.len() == subgrid.len() && light_total(&k) == light_total(&subgrid)
                    })
                    .find(|&&(ref k, _)| variants(&k).iter().any(|p| *p == subgrid))
                    .unwrap()
                    .1
                    .clone();

                for x in 0..new_val.len() {
                    for y in 0..new_val.len() {
                        newgrid[i * new_val.len() + x][j * new_val.len() + y] = new_val[x][y];
                    }
                }
            }
        }
        grid = newgrid;
        if n == 5 - 1 {
            println!("Lights after 5 iterations: {}", light_total(&grid));
        }
    }
    println!("Lights after 18 iterations: {}", light_total(&grid));
}
