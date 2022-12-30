use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn knot_hash(input: &[u8]) -> Vec<u8> {
    let mut rope: Vec<usize> = (0..256).collect();
    let mut pos: usize = 0;

    let additional = vec![17, 31, 73, 47, 23];
    let input2 = [input, &additional[..]].concat();

    for r in 0..64 {
        for (skip, &ins) in input2.iter().enumerate() {
            let subrope: Vec<_> = rope
                .iter()
                .cycle()
                .skip(pos)
                .take(ins as usize)
                .cloned()
                .collect();
            let subrope_r: Vec<_> = subrope.iter().rev().collect();
            let len = rope.len();
            for (i, &&r) in subrope_r.iter().enumerate() {
                rope[(pos + i) % len] = r;
            }
            pos = (pos + ins as usize + skip + (r * input2.len())) % 256;
        }
    }
    return rope
        .chunks(16)
        .map(|c| c.iter().fold(0, |a, b| a ^ b))
        .map(|c| c as u8)
        .collect();
}

fn flood_fill(grid: &Vec<Vec<u8>>, x: usize, y: usize, filled: &mut Vec<(usize, usize)>) {
    if grid[x][y] != 1 {
        return;
    }
    if filled.iter().any(|&t| t == (x, y)) {
        return;
    }
    filled.push((x, y));
    let modcoord: &[i32] = &[-1, 0, 1];
    for i in modcoord {
        for j in modcoord {
            if i.abs() == j.abs() {
                continue;
            }
            let n_x = x as i32 + i;
            if n_x < 0 || n_x >= grid.len() as i32 {
                continue;
            }
            let n_y = y as i32 + j;
            if n_y < 0 || n_y >= grid.len() as i32 {
                // assume square
                continue;
            }
            flood_fill(grid, n_x as usize, n_y as usize, filled);
        }
    }
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input: String = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let grid: Vec<Vec<u8>> = (0..128)
        .map(|i| {
            let input_str = input.clone() + "-" + &format!("{}", i);
            knot_hash(input_str.as_bytes())
                .iter()
                .map(|e| format!("{:08b}", e).into_bytes())
                .flat_map(|v| v.into_iter())
                .map(|b| b - b'0')
                .collect()
        })
        .collect();

    let used_squares: usize = grid
        .iter()
        .map(|s| s.iter().filter(|&&c| c == 1).count())
        .sum();
    println!("Used squares: {:?}", used_squares);

    let mut groups: Vec<Vec<(usize, usize)>> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if groups.iter().flat_map(|v| v.iter()).any(|&t| t == (i, j)) {
                continue;
            }
            if grid[i][j] != 1 {
                continue;
            }
            groups.push(Vec::new());
            let len = groups.len();
            flood_fill(&grid, i, j, groups.get_mut(len - 1).unwrap());
        }
    }
    println!("Number of regions: {}", groups.len());
    //    println!("{:?}", groups);
}
