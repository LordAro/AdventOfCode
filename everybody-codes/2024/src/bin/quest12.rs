use std::collections::HashMap;
use std::fs;
use std::io;

// standard graph orientation - 0, 0 in bottom left, x horizontal
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
enum Type {
    Normal,
    Hard,
    Meteor, // lol
}

fn get_targets(input: &str) -> Vec<(Coord, Type)> {
    // assume that catapult A is at 0,0
    let input_height = input.lines().count() - 1;
    input
        .lines()
        .enumerate()
        .flat_map(|(str_y, l)| {
            let y = input_height - (str_y + 1);
            l.chars()
                .skip(1)
                .enumerate()
                .filter(|(_, c)| *c == 'T' || *c == 'H')
                .map(move |(x, c)| {
                    (
                        Coord { x, y },
                        if c == 'T' { Type::Normal } else { Type::Hard },
                    )
                })
        })
        .collect()
}

fn get_catapult_landing_x(start_pos: usize, power: usize) -> usize {
    start_pos + 3 * power
}

fn get_ranking_value_sum(targets: &[(Coord, Type)]) -> usize {
    let furthest_target = targets.iter().max_by_key(|ct| ct.0).unwrap();
    let furthest_target_x = furthest_target.0.x + furthest_target.0.y;

    // target x -> pos, power
    let mut firing_solutions: HashMap<usize, (usize, usize)> = HashMap::new();
    for start_catapult in 0..3 {
        for power in 1.. {
            let landing_pos = get_catapult_landing_x(start_catapult, power);
            firing_solutions.insert(landing_pos, (start_catapult, power));
            if landing_pos >= furthest_target_x {
                break;
            }
        }
    }
    targets
        .iter()
        .map(|(target, type_)| {
            let (catapult, power) = firing_solutions.get(&(target.x + target.y)).unwrap();
            (catapult + 1) * power * if *type_ == Type::Normal { 1 } else { 2 }
        })
        .sum()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input = fs::read_to_string(p1_input_filename)?;
    let p1_targets = get_targets(&p1_input);
    let p1_ranking_value_sum = get_ranking_value_sum(&p1_targets);
    println!("P1: Ranking value sum: {p1_ranking_value_sum}");

    let p2_input = fs::read_to_string(p2_input_filename)?;
    let p2_targets = get_targets(&p2_input);
    let p2_ranking_value_sum = get_ranking_value_sum(&p2_targets);
    println!("P2: Ranking value sum: {p2_ranking_value_sum}");

    let p3_input = fs::read_to_string(p3_input_filename)?;
    let p3_targets: Vec<_> = p3_input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(' ').unwrap();
            (
                Coord {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                },
                Type::Meteor,
            )
        })
        .collect();

    {
        let furthest_target = p3_targets.iter().max_by_key(|ct| ct.0).unwrap();
        let furthest_target_x = furthest_target.0.x - furthest_target.0.y;

        // target x -> pos, power
        let mut firing_solutions: HashMap<usize, (usize, usize)> = HashMap::new();
        for start_catapult in 0..3 {
            for power in 1.. {
                let landing_pos = get_catapult_landing_x(start_catapult, power);
                firing_solutions.insert(landing_pos, (start_catapult, power));
                if landing_pos >= furthest_target_x {
                    break;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input_str = ".............
.C...........
.B......T....
.A......T.T..
=============";
        let target_positions = get_targets(input_str);
        let expected_targets = [
            (Coord { x: 7, y: 1 }, Type::Normal),
            (Coord { x: 7, y: 0 }, Type::Normal),
            (Coord { x: 9, y: 0 }, Type::Normal),
        ];
        assert_eq!(target_positions, expected_targets);
    }

    #[test]
    fn ex1b() {
        // B, 5
        assert_eq!(get_catapult_landing_x(1, 5), 16);
        // C, 3
        assert_eq!(get_catapult_landing_x(2, 3), 11);
    }

    #[test]
    fn ex1c() {
        let input_str = ".............
.C...........
.B......T....
.A......T.T..
=============";
        let target_positions = get_targets(input_str);
        let ranking_value_sum = get_ranking_value_sum(&target_positions);
        assert_eq!(ranking_value_sum, 13);
    }
}
