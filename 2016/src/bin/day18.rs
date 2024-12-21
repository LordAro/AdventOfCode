use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn build_next_line(line: &str) -> String {
    // edges treated as 'safe'
    [b".", line.as_bytes(), b"."]
        .concat()
        .windows(3)
        .map(|slice| match slice {
            [b'^', b'^', b'.'] => '^',
            [b'.', b'^', b'^'] => '^',
            [b'^', b'.', b'.'] => '^',
            [b'.', b'.', b'^'] => '^',
            [_, _, _] => '.',
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let mut lines = vec![input];
    while lines.len() < 400000 {
        lines.push(build_next_line(lines.last().unwrap()));
    }

    let num_safe40 = lines
        .iter()
        .take(40)
        .flat_map(|s| s.chars())
        .filter(|&e| e == '.')
        .count();
    println!("Number of safe tiles (40 lines): {}", num_safe40);

    let num_safe = lines
        .iter()
        .flat_map(|s| s.chars())
        .filter(|&e| e == '.')
        .count();
    println!("Number of safe tiles (400000 lines): {}", num_safe);
}
