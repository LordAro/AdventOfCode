use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let input: usize = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();

    let abslimit = (1usize..)
        .skip_while(|x| x % 2 == 0 || x.pow(2) < input) // Corner powers are odd
        .next()
        .unwrap();
    let limit = (abslimit - 1) / 2;

    // OEIS A039823 a(n) = ceiling( (n^2 + n + 2)/4 )
    let centralpoint = (1usize..)
        .map(|n| ((n.pow(2) + n + 2) as f64 / 4.0).ceil() as usize)
        .skip_while(|n| *n < input - limit)
        .next()
        .unwrap();

    println!("Manhattan distance: {}", limit + input - centralpoint);

    let mut grid = vec![vec![0; abslimit]; abslimit];
    let middle = (limit, limit);
    let surround: Vec<(i32, i32)> = vec![
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    // See OEIS A141481 for algorithm
    grid[middle.0][middle.1] = 1;
    for n in 1usize..abslimit {
        let g = (n as f64).sqrt() as i32;
        let r = (g + g % 2) / 2;
        let q = 4 * r.pow(2);
        let d = (n as i32) - q;
        let point = if n <= (q - 2 * r) as usize {
            (d + 3 * r, r)
        } else if n <= q as usize {
            (r, -d - r)
        } else if n <= (q + 2 * r) as usize {
            (r - d, -r)
        } else {
            (-r, d - 3 * r)
        };
        let norm = (
            (point.0 + middle.0 as i32) as usize,
            (point.1 + middle.1 as i32) as usize,
        );
        // Sum surrounding grid points
        grid[norm.0][norm.1] = surround.iter().fold(0, |acc, &t| {
            acc + grid[(norm.0 as i32 + t.0) as usize][(norm.1 as i32 + t.1) as usize]
        });
        if grid[norm.0][norm.1] >= input {
            println!(
                "First sum value greater than input: {}",
                grid[norm.0][norm.1]
            );
            break;
        }
    }
}
