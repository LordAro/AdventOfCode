use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::io;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let t = a;
        a = b;
        b = t % b;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}

fn parse_wheels<const MUZZLE: bool>(input: &[&str], num_cols: usize) -> Vec<Vec<[u8; 3]>> {
    let mut wheels: Vec<Vec<[u8; 3]>> = vec![vec![]; num_cols];
    for l in input.iter() {
        let byte_line = l.as_bytes();
        for (col_no, wheel) in wheels.iter_mut().enumerate() {
            let s_index = col_no * 4; // cat (3) + space
            if s_index < byte_line.len() && byte_line[s_index] != b' ' {
                let cat_bytes = if MUZZLE {
                    [
                        byte_line[s_index],
                        byte_line[s_index + 1],
                        byte_line[s_index + 2],
                    ]
                } else {
                    [byte_line[s_index], 0, byte_line[s_index + 2]]
                };
                wheel.push(cat_bytes);
            }
        }
    }
    wheels
}

fn push_left_lever(state: &[usize], sizes: &[usize]) -> Vec<usize> {
    // lots of casts to avoid underflow
    state
        .iter()
        .enumerate()
        .map(|(col, pos)| (*pos as i64 - 1).rem_euclid(sizes[col] as i64) as usize)
        .collect()
}

fn pull_left_lever(state: &[usize], sizes: &[usize]) -> Vec<usize> {
    state
        .iter()
        .enumerate()
        .map(|(col, pos)| (pos + 1).rem_euclid(sizes[col]))
        .collect()
}

fn pull_right_lever(state: &[usize], sizes: &[usize], incrs: &[usize]) -> Vec<usize> {
    state
        .iter()
        .enumerate()
        .map(|(col, pos)| (pos + incrs[col]).rem_euclid(sizes[col]))
        .collect()
}

fn get_num_bytecoins(wheels: &[[u8; 3]]) -> usize {
    let counts = wheels
        .iter()
        .flat_map(|cat| cat.iter())
        .filter(|b| **b != 0)
        .counts();
    // 3 matching symbols is 1 coin, 2 or fewer is 0
    let num_bytecoins: usize = counts.values().map(|v| v.max(&2) - 2).sum();
    num_bytecoins
}

fn wheel_positions_to_values(wheels: &[Vec<[u8; 3]>], positions: &[usize]) -> Vec<[u8; 3]> {
    wheels
        .iter()
        .enumerate()
        .map(|(wheel_n, wheel)| wheel[positions[wheel_n]])
        .collect()
}

fn search<const LIMIT: usize>(
    cache: &mut HashMap<(Vec<usize>, usize, usize), (usize, usize)>,
    wheels: &[Vec<[u8; 3]>],
    wheel_lens: &[usize],
    incrs: &[usize],
    start_state: &[usize],
    current_score: usize,
    pull_n: usize,
) -> (usize, usize) {
    if pull_n == LIMIT {
        return (current_score, current_score);
    }
    if let Some(scores) = cache.get(&(start_state.to_vec(), pull_n, current_score)) {
        return *scores;
    }
    let post_state = pull_right_lever(start_state, wheel_lens, incrs);
    let push_post_state = push_left_lever(&post_state, wheel_lens);
    let pull_post_state = pull_left_lever(&post_state, wheel_lens);

    let push_val = get_num_bytecoins(&wheel_positions_to_values(wheels, &push_post_state));
    let n_val = get_num_bytecoins(&wheel_positions_to_values(wheels, &post_state));
    let pull_val = get_num_bytecoins(&wheel_positions_to_values(wheels, &pull_post_state));

    let push_rec_val = search::<LIMIT>(
        cache,
        wheels,
        wheel_lens,
        incrs,
        &push_post_state,
        current_score + push_val,
        pull_n + 1,
    );
    let n_rec_val = search::<LIMIT>(
        cache,
        wheels,
        wheel_lens,
        incrs,
        &post_state,
        current_score + n_val,
        pull_n + 1,
    );
    let pull_rec_val = search::<LIMIT>(
        cache,
        wheels,
        wheel_lens,
        incrs,
        &pull_post_state,
        current_score + pull_val,
        pull_n + 1,
    );

    let min = (push_rec_val.0).min(n_rec_val.0).min(pull_rec_val.0);
    let max = (push_rec_val.1).max(n_rec_val.1).max(pull_rec_val.1);
    cache.insert((start_state.to_vec(), pull_n, current_score), (min, max));
    (min, max)
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    {
        let p1_input = fs::read_to_string(p1_input_filename)?;
        let position_increments: Vec<_> = p1_input
            .lines()
            .next()
            .expect("Unexpected input format")
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let wheels_str: Vec<_> = p1_input.lines().skip(2).collect();
        let wheels = parse_wheels::<true>(&wheels_str, position_increments.len());

        let wheels_at_100: Vec<_> = wheels
            .iter()
            .enumerate()
            .map(|(col_n, wheel)| {
                let cat = wheel[(100 * position_increments[col_n]) % wheel.len()];
                String::from_utf8(cat.to_vec()).unwrap()
            })
            .collect();
        let string_rep = wheels_at_100.iter().join(" ");

        println!("P1: Wheels after 100 pulls: {string_rep}");
    }

    {
        let p2_input = fs::read_to_string(p2_input_filename)?;
        let position_increments: Vec<_> = p2_input
            .lines()
            .next()
            .expect("Unexpected input format")
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let wheels_str: Vec<_> = p2_input.lines().skip(2).collect();
        let wheels = parse_wheels::<false>(&wheels_str, position_increments.len());

        let mut coin_counts = vec![];
        let mut states: Vec<Vec<usize>> = vec![];
        for i in 0.. {
            let wheels_state: Vec<_> = wheels
                .iter()
                .enumerate()
                .map(|(col_n, wheel)| (i * position_increments[col_n]) % wheel.len())
                .collect();

            let counts = wheels_state
                .iter()
                .enumerate()
                .flat_map(|(i, pos)| wheels[i][*pos].iter())
                .filter(|b| **b != 0)
                .counts();
            let num_bytecoins: usize = counts.values().map(|v| v.max(&2) - 2).sum();
            coin_counts.push(num_bytecoins);

            if states.contains(&wheels_state) {
                // Loop found
                break;
            }
            states.push(wheels_state);
        }

        let lcm = wheels.iter().map(|wheel| wheel.len()).reduce(lcm).unwrap();
        let rem = 202420242024 % lcm;
        let loop_total: usize = coin_counts.iter().sum();
        let total =
            202420242024 / lcm * loop_total + coin_counts.iter().skip(1).take(rem).sum::<usize>();

        println!("P2: Total bytecoints up to 202420242024: {total}");
    }

    {
        let p3_input = fs::read_to_string(p3_input_filename)?;

        let position_increments: Vec<_> = p3_input
            .lines()
            .next()
            .expect("Unexpected input format")
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let wheels_str: Vec<_> = p3_input.lines().skip(2).collect();
        let wheels = parse_wheels::<false>(&wheels_str, position_increments.len());
        let wheel_lens: Vec<_> = wheels.iter().map(|w| w.len()).collect();

        let wheel_positions = vec![0usize; position_increments.len()];
        let mut cache = HashMap::default();
        let dfs = search::<256>(
            &mut cache,
            &wheels,
            &wheel_lens,
            &position_increments,
            &wheel_positions,
            0,
            0,
        );

        println!("P3: Max/min bytecoins after 256 pulls: {} {}", dfs.1, dfs.0);
    }
    Ok(())
}
