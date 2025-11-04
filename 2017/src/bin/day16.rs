use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// rotate right
fn spin_transform<T: Clone>(input: &[T], idx: usize) -> Vec<T> {
    [&input[(input.len() - idx)..], &input[..(input.len() - idx)]].concat()
}

fn swap_transform<T: Clone>(input: &[T], a: usize, b: usize) -> Vec<T> {
    let mut output = input.to_owned();
    output.swap(a, b);
    output
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let mut programs: Vec<_> = "abcdefghijklmnop".chars().collect();

    let input: Vec<(char, u8, u8)> =
        BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|ins| {
                // initial parse
                let mut ins_it = ins.chars();
                let cmd = ins_it.next().unwrap();
                let op1;
                let op2;
                match cmd {
                    's' => {
                        op1 = ins_it.collect::<String>().parse().unwrap();
                        op2 = 0;
                    }
                    'x' => {
                        let idxs = ins_it.collect::<String>();
                        let mut idxs_it = idxs.split('/');
                        op1 = idxs_it.next().unwrap().parse().unwrap();
                        op2 = idxs_it.next().unwrap().parse().unwrap();
                    }
                    'p' => {
                        let ps: Vec<_> = ins_it.collect();
                        op1 = *ps.first().unwrap() as u8;
                        op2 = *ps.get(2).unwrap() as u8;
                    }
                    _ => panic!("Unrecognised instruction"),
                }
                (cmd, op1, op2)
            })
            .collect();

    // Merge program swaps together
    let mut prog_swaps = HashMap::new();
    for (_, a, b) in input.iter().cloned().filter(|&(cmd, _, _)| cmd == 'p') {
        let key_a = prog_swaps
            .iter()
            .filter(|&(_, &v)| v == a)
            .map(|(&k, _)| k)
            .next()
            .unwrap_or(a);
        let key_b = prog_swaps
            .iter()
            .filter(|&(_, &v)| v == b)
            .map(|(&k, _)| k)
            .next()
            .unwrap_or(b);
        prog_swaps.insert(key_a, b);
        prog_swaps.insert(key_b, a);
    }

    // Merge spins and swaps together into a single transform array
    let transformed_input: Vec<_> = input
        .iter()
        .cloned()
        .filter(|&(cmd, _, _)| cmd != 'p')
        .fold((0..programs.len() as u8).collect(), |ordering, x| {
            match x.0 {
                's' => spin_transform(&ordering, x.1 as usize),
                'x' => swap_transform(&ordering, x.1 as usize, x.2 as usize),
                _ => unreachable!(),
            }
        });

    let mut seen_program_orderings: HashMap<Vec<char>, usize> = HashMap::new();

    for d in 0..1_000_000_000 {
        let mut new_programs = programs.clone();
        for (&a, &b) in &prog_swaps {
            let idx_a = programs.iter().position(|&c| c == a as char).unwrap();
            new_programs[idx_a] = b as char;
        }
        programs = new_programs;
        programs = transformed_input
            .iter()
            .map(|&i| programs[i as usize])
            .collect();

        // found a loop
        if let Some(seen_d) = seen_program_orderings.get(&programs) {
            let rem = (1_000_000_000 + seen_d) % d;
            println!(
                "Final dance position: {}",
                seen_program_orderings
                    .iter()
                    .find(|&(_, &v)| v == rem - 1)
                    .unwrap()
                    .0
                    .iter()
                    .collect::<String>()
            );
            break;
        } else {
            seen_program_orderings.insert(programs.clone(), d);
        }
        if d == 0 {
            println!("Dance position 1: {}", programs.iter().collect::<String>());
        }
    }
}
