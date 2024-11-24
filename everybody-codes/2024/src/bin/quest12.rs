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
struct IntersectInfo {
    start_pos: usize,
    power: usize,
    altitude: usize,
    time: usize, // debugging only
}

#[derive(Debug, PartialEq)]
enum Type {
    Normal,
    Hard,
}

fn get_targets(input: &str) -> Vec<(Coord, Type)> {
    // assume that catapult A is at 0,0
    let input_height = input.lines().count() - 1;
    input
        .lines()
        .enumerate()
        .take_while(|(str_y, _)| *str_y < input_height) // skip last line
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

fn get_projectile_pos_at_time(start_pos: usize, power: usize, time: usize) -> Option<Coord> {
    if time <= power {
        Some(Coord {
            x: time,
            y: start_pos + time,
        })
    } else if time <= power * 2 {
        Some(Coord {
            x: time,
            y: start_pos + power,
        })
    } else {
        (start_pos + 3 * power)
            .checked_sub(time)
            .map(|y| Coord { x: time, y })
    }
}

fn get_projectile_meteor_intersect_altitude(
    start_pos: usize,
    power: usize,
    meteor: Coord,
) -> Option<usize> {
    let meteor_coords = (0..).map_while(|t| {
        meteor
            .y
            .checked_sub(t)
            .and_then(|y| meteor.x.checked_sub(t).map(|x| Coord { x, y }))
    });
    let projectile_coords = (0..).map_while(|t| get_projectile_pos_at_time(start_pos, power, t));

    projectile_coords
        .zip(meteor_coords)
        .find(|(p, m)| p == m)
        .map(|(p, _)| p.y)
}

fn find_meteor_intersect(meteor_start: Coord) -> Option<IntersectInfo> {
    // never changes through multiple time steps
    let meteor_landing_x = meteor_start.x.saturating_sub(meteor_start.y);

    // can't step any further than this
    // Find the first result we get at this time step.
    // If we've found a solution, there can't be any better ones at later time values as the
    // altitude will be lower, so return immediately
    (0..usize::min(meteor_start.x, meteor_start.y)).find_map(|time| {
        let meteor = Coord {
            x: meteor_start.x - time,
            y: meteor_start.y - time,
        };

        // For all 3 catapults, find the first intersect point
        (0..3).find_map(move |catapult| {
            // get a power level that approximates the range we can intersect with the meteor
            // power_end is overkill, but necessary for the few meteors that won't actually land
            let power_start = meteor_landing_x.saturating_sub(catapult) / 3;
            let power_end = (meteor.x + meteor.y - catapult) / 3;

            (power_start..=power_end)
                // first result at this power level is the best by construction
                .find_map(move |power| {
                    get_projectile_meteor_intersect_altitude(catapult, power, meteor).map(|y| {
                        IntersectInfo {
                            start_pos: catapult,
                            power,
                            altitude: y,
                            time,
                        }
                    })
                })
        })
    })
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
            let (catapult, power) = firing_solutions[&(target.x + target.y)];
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
    let p3_min_rank_score: usize = p3_input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(' ').unwrap();
            Coord {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .map(|meteor| {
            let ii = find_meteor_intersect(meteor).unwrap();
            (ii.start_pos + 1) * ii.power
        })
        .sum();

    println!("P3: Minimum ranking score to destroy all meteors: {p3_min_rank_score}");

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

    #[test]
    fn ex3a() {
        // C1
        let positions: Vec<_> = (0..)
            .map_while(|t| get_projectile_pos_at_time(2, 1, t))
            .collect();
        let expected = vec![
            Coord { x: 0, y: 2 },
            Coord { x: 1, y: 3 },
            Coord { x: 2, y: 3 },
            Coord { x: 3, y: 2 },
            Coord { x: 4, y: 1 },
            Coord { x: 5, y: 0 },
        ];
        assert_eq!(positions, expected);

        // C3
        let positions: Vec<_> = (0..)
            .map_while(|t| get_projectile_pos_at_time(2, 3, t))
            .collect();
        let expected = vec![
            Coord { x: 0, y: 2 },
            Coord { x: 1, y: 3 },
            Coord { x: 2, y: 4 },
            Coord { x: 3, y: 5 },
            Coord { x: 4, y: 5 },
            Coord { x: 5, y: 5 },
            Coord { x: 6, y: 5 },
            Coord { x: 7, y: 4 },
            Coord { x: 8, y: 3 },
            Coord { x: 9, y: 2 },
            Coord { x: 10, y: 1 },
            Coord { x: 11, y: 0 },
        ];
        assert_eq!(positions, expected);
    }

    #[test]
    fn ex3b() {
        // C1
        assert_eq!(
            get_projectile_meteor_intersect_altitude(2, 1, Coord { x: 6, y: 5 }),
            Some(2)
        );
    }

    #[test]
    fn ex3c() {
        // Finds A2
        assert_eq!(
            find_meteor_intersect(Coord { x: 6, y: 5 }),
            Some(IntersectInfo {
                start_pos: 0,
                power: 2,
                altitude: 2,
                time: 0,
            })
        );

        assert_eq!(
            find_meteor_intersect(Coord { x: 6, y: 7 }),
            // C2 also valid, with same rank score
            Some(IntersectInfo {
                start_pos: 1,
                power: 3,
                altitude: 4,
                time: 0,
            })
        );

        assert_eq!(
            find_meteor_intersect(Coord { x: 10, y: 5 }),
            Some(IntersectInfo {
                start_pos: 2,
                power: 1,
                altitude: 0,
                time: 0,
            })
        );

        // Needs delay
        assert_eq!(
            find_meteor_intersect(Coord { x: 5, y: 5 }),
            Some(IntersectInfo {
                start_pos: 0,
                power: 2,
                altitude: 2,
                time: 1,
            })
        );
    }
}
