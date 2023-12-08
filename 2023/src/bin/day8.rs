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
    let mut cur_loc = start_pos;
    let mut steps = 0;
    while !exit_pred(cur_loc) {
        let next_loc = map.get(cur_loc).unwrap();
        let next_move = instructions
            .chars()
            .nth(steps % instructions.len())
            .unwrap();
        cur_loc = match next_move {
            'L' => next_loc.0,
            'R' => next_loc.1,
            _ => unreachable!(),
        };
        steps += 1;
    }
    steps
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

    let instr = input_data[0].clone();
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

    //instr.chars().cycle().take_while(|x| x != "ZZZ").count()
    let p1_steps = find_exit(&instr, &map, "AAA", |p| p == "ZZZ");
    println!("Number of steps to reach ZZZ: {}", p1_steps);

    let start_locs: Vec<_> = map.keys().filter(|&&k| k.ends_with('A')).copied().collect();
    let step_counts: Vec<usize> = start_locs
        .iter()
        .map(|pos| find_exit(&instr, &map, pos, |p| p.ends_with('Z')))
        .collect();

    let steps_needed: usize = step_counts.iter().fold(1, |a, b| num::integer::lcm(a, *b));

    println!("Number of simultaneous steps needed: {}", steps_needed);

    Ok(())
}
