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

fn get_adjacent_coords(c: Coord, num: u32, max_size: Coord) -> Vec<Coord> {
    let num_digits = num.ilog10() as usize + 1;
    let mut ret = vec![];
    // before
    if c.x > 0 {
        if c.y > 0 {
            ret.push(Coord {
                x: c.x - 1,
                y: c.y - 1,
            });
        }
        ret.push(Coord { x: c.x - 1, y: c.y });
        if c.y < max_size.y {
            ret.push(Coord {
                x: c.x - 1,
                y: c.y + 1,
            });
        }
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
        //ret.push(Coord { x: c.x + n, y: c.y });
        if c.y < max_size.y {
            ret.push(Coord {
                x: c.x + n,
                y: c.y + 1,
            });
        }
    }
    // after
    if c.x + num_digits - 1 < max_size.x {
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
        if c.y < max_size.y {
            ret.push(Coord {
                x: c.x + num_digits,
                y: c.y + 1,
            });
        }
    }
    ret
}

fn main() -> io::Result<()> {
    let input_data: Vec<String> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| l.unwrap().parse().unwrap())
    .collect();

    let numbers: Vec<(Coord, u32)> = input_data
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            // Convert the line into an enumerated (idx, char) vector
            // Then split on digit boundaries (and throw away all the empties)
            // Then reparse the digits into a number along with the coord of the first digit
            let enumerated_line = line.chars().enumerate().collect::<Vec<_>>();
            enumerated_line
                .split(|(_, c)| !c.is_ascii_digit())
                .filter(|&arr| !arr.is_empty())
                .map(move |arr| {
                    (
                        Coord { x: arr[0].0, y },
                        arr.iter()
                            .map(|(_, c)| c)
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let symbols: HashMap<Coord, char> = input_data
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.' && !c.is_ascii_digit())
                .map(move |(x, c)| (Coord { x, y }, c))
        })
        .collect();

    let max_coord = Coord {
        x: input_data[0].len(),
        y: input_data.len(),
    };

    let engine_id_sum: u32 = numbers
        .iter()
        .filter(|(c, num)| {
            get_adjacent_coords(*c, *num, max_coord)
                .iter()
                .any(|coord| symbols.contains_key(coord))
        })
        .map(|(_, num)| num)
        .sum();

    println!("Sum of all valid engine IDs: {}", engine_id_sum);

    let potential_gear_coords: Vec<_> = numbers
        .iter()
        .flat_map(|(c, num)| {
            get_adjacent_coords(*c, *num, max_coord)
                .iter()
                .filter(|coord| symbols.get(coord).is_some_and(|&s| s == '*'))
                .map(|coord| (*coord, *num))
                .collect::<Vec<(Coord, u32)>>()
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
