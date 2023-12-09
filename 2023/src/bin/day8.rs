use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

extern crate num;

fn find_exit<F>(
    instructions: &str,
    map: &HashMap<&str, (&str, &str)>,
    start_pos: &str,
    exit_pred: F,
) -> usize
where
    F: Fn(&str) -> bool,
{
    instructions
        .chars()
        .cycle()
        .scan(start_pos, |cur_loc, next_move| {
            // stop when we get to the end
            if exit_pred(cur_loc) {
                return None;
            }
            // update state to next location
            let next_loc_pair = map.get(cur_loc).unwrap();
            *cur_loc = match next_move {
                'L' => next_loc_pair.0,
                'R' => next_loc_pair.1,
                _ => unreachable!(),
            };

            // don't actually care about the iterator, everything is stored in cur_loc state var
            Some(1)
        })
        .count()
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
    .map(|l| l.unwrap())
    .collect();

    let instr = &input_data[0];
    let map: HashMap<&str, (&str, &str)> = input_data
        .iter()
        .skip(2)
        .map(|l| {
            let k = &l[0..3];
            let v1 = &l[7..10];
            let v2 = &l[12..15];
            (k, (v1, v2))
        })
        .collect();

    let p1_steps = find_exit(instr, &map, "AAA", |p| p == "ZZZ");
    println!("Number of steps to reach ZZZ: {}", p1_steps);

    let start_locs: Vec<_> = map.keys().filter(|&&k| k.ends_with('A')).copied().collect();
    let step_counts: Vec<usize> = start_locs
        .iter()
        .map(|pos| find_exit(instr, &map, pos, |p| p.ends_with('Z')))
        .collect();

    let steps_needed: usize = step_counts.iter().fold(1, |a, b| num::integer::lcm(a, *b));

    println!("Number of simultaneous steps needed: {}", steps_needed);

    Ok(())
}
