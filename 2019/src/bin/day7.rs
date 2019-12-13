use std::cmp::max;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

extern crate permutohedron;
use permutohedron::Heap;

extern crate advent_of_code;
use advent_of_code::intcode;

fn main() -> io::Result<()> {
    let program_str = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .next()
    .unwrap()
    .unwrap();

    let program = intcode::read_input(&program_str);

    let mut phases = vec![0, 1, 2, 3, 4];
    let phase_heap = Heap::new(&mut phases);
    let mut max_output = isize::min_value();
    for permutation in phase_heap {
        let mut input_val = 0;
        for phase in permutation {
            let mut mach = intcode::Machine::new(&program, &[phase, input_val]);
            let output = mach.run_until_output();
            input_val = output.unwrap();
        }
        max_output = max(max_output, input_val);
    }
    println!("Maximum output {}", max_output);

    let mut phases: Vec<isize> = vec![5, 6, 7, 8, 9];
    let phase_heap = Heap::new(&mut phases);
    let mut max_output = isize::min_value();
    for permutation in phase_heap {
        // Initialise machines
        let mut machines = Vec::with_capacity(5);
        for phase in permutation {
            machines.push(intcode::Machine::new(&program, &[phase]));
        }
        machines[0].push_input(0);
        let mut last_output = machines[0].run_until_output();

        // Run machines in a loop until one halts
        let mut i = 1;
        while last_output.is_some() {
            machines[i].push_input(last_output.unwrap());
            let output = machines[i].run_until_output();
            if output.is_none() {
                break;
            }
            last_output = output;
            i = (i + 1) % machines.len();
        }
        max_output = max(max_output, last_output.unwrap());
    }
    println!("Maximum output (feedback mode): {}", max_output);

    Ok(())
}
