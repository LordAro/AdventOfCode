use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

fn get_adjacent_coords(c: &Coord, num: &u32) -> Vec<Coord> {
    let num_digits = num.ilog10() as usize + 1;
    let mut ret = vec![];

    // don't need to worry about exceeding the maximum size of the grid, coordinates are all in hashmaps

    // before
    if c.x > 0 {
        if c.y > 0 {
            ret.push(Coord {
                x: c.x - 1,
                y: c.y - 1,
            });
        }
        ret.push(Coord { x: c.x - 1, y: c.y });
        ret.push(Coord {
            x: c.x - 1,
            y: c.y + 1,
        });
    }

    // during
    for n in 0..num_digits {
        if c.y > 0 {
            ret.push(Coord {
                x: c.x + n,
                y: c.y - 1,
            });
        }
        // don't need to include the coordinates of the number itself
        ret.push(Coord {
            x: c.x + n,
            y: c.y + 1,
        });
    }

    // after
    if c.y > 0 {
        ret.push(Coord {
            x: c.x + num_digits,
            y: c.y - 1,
        });
    }
    ret.push(Coord {
        x: c.x + num_digits,
        y: c.y,
    });
    ret.push(Coord {
        x: c.x + num_digits,
        y: c.y + 1,
    });

    ret
}

fn main() -> io::Result<()> {
    let input_data: Vec<Vec<u8>> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| l.unwrap().bytes().collect())
    .collect();

    let numbers: Vec<(Coord, u32)> = input_data
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            // Convert the line into an enumerated (idx, char) vector
            // Then split on digit boundaries (and throw away all the empties)
            // Then reparse the digits into a number along with the coord of the first digit
            let enumerated_line = line.iter().enumerate().collect::<Vec<_>>();
            enumerated_line
                .split(|(_, c)| !c.is_ascii_digit())
                .filter(|&arr| !arr.is_empty())
                .map(move |arr| {
                    (
                        Coord { x: arr[0].0, y },
                        arr.iter()
                            .fold(0u32, |acc, &(_, c)| acc * 10 + (c - b'0') as u32),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let symbols: HashMap<Coord, u8> = input_data
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &c)| c != b'.' && !c.is_ascii_digit())
                .map(move |(x, &c)| (Coord { x, y }, c))
        })
        .collect();

    let engine_id_sum: u32 = numbers
        .iter()
        .filter(|&(c, num)| {
            get_adjacent_coords(c, num)
                .iter()
                .any(|coord| symbols.contains_key(coord))
        })
        .map(|(_, num)| num)
        .sum();

    println!("Sum of all valid engine IDs: {}", engine_id_sum);

    let potential_gear_coords: Vec<_> = numbers
        .iter()
        .flat_map(|(c, num)| {
            get_adjacent_coords(c, num)
                .iter()
                .filter(|coord| symbols.get(coord).is_some_and(|&s| s == b'*'))
                .map(|coord| (*coord, *num))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut gear_counter: HashMap<Coord, Vec<u32>> = HashMap::new();
    for &(symbol_pos, num) in &potential_gear_coords {
        gear_counter.entry(symbol_pos).or_default().push(num);
    }

    let gear_ratio_sum: u32 = gear_counter
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();

    println!("Sum of all gear ratios: {}", gear_ratio_sum);

    Ok(())
}
