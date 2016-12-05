use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    // let valids = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
    //    .lines()
    //    .map(|l| l.unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<i32>>())
    //    .filter(|t| t[0] + t[1] > t[2] && t[0] + t[2] > t[1] && t[1] + t[2] > t[0])
    //    .count();

    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap());

    let triangles = input.lines()
        .map(|l| l.unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<i32>>());

    let mut count = 0;
    let mut count2 = 0;
    let mut trans = vec![vec![]; 3];
    for t in triangles {
        if t[0] + t[1] > t[2] && t[0] + t[2] > t[1] && t[1] + t[2] > t[0] {
            count += 1
        }
        for j in 0..3 {
            trans[j].push(t[j])
        }
        if trans[0].len() == 3 {
            for j in 0..3 {
                if trans[j][0] + trans[j][1] > trans[j][2]
                    && trans[j][0] + trans[j][2] > trans[j][1]
                    && trans[j][1] + trans[j][2] > trans[j][0] {
                    count2 += 1;
                }
            trans[j].clear();
            }
        }
    }
    println!("Number of valid row triangles: {}", count);
    println!("Number of valid column triangles: {}", count2);
}
