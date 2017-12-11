use std::fs::File;
use std::cmp;
use std::env;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn distance_from_origin(point: &Point) -> i32 {
    (point.x.abs() + point.y.abs() + point.z.abs()) / 2
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input: Vec<_> = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|d| d.to_string())
        .collect();

    let mut max_distance = 0;
    let mut point = Point { x: 0, y: 0, z: 0 };
    for dir in &input {
        match dir.as_str() {
            "n" => {
                point.y += 1;
                point.z -= 1;
            }
            "s" => {
                point.y -= 1;
                point.z += 1;
            }
            "ne" => {
                point.x += 1;
                point.z -= 1;
            }
            "sw" => {
                point.x -= 1;
                point.z += 1;
            }
            "nw" => {
                point.x -= 1;
                point.y += 1;
            }
            "se" => {
                point.x += 1;
                point.y -= 1;
            }
            _ => panic!("Unknown direction {}", dir),
        }
        max_distance = cmp::max(distance_from_origin(&point), max_distance);
    }
    println!("Final distance: {}", distance_from_origin(&point));
    println!("Maximum distance: {}", max_distance);
}
