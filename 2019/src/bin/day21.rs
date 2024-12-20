use std::env;
use std::fs;
use std::io;

use advent_of_code::intcode;
use itertools::{iproduct, Itertools};

fn run_springdroid_program(
    springdroid_os: &[intcode::Word],
    instrs: &[&str],
    start_cmd: &str,
    debug: bool,
) -> Option<isize> {
    let springdroid_program = format!("{}\n{}\n", instrs.join("\n"), start_cmd);
    let springdroid_program: Vec<_> = springdroid_program
        .as_bytes()
        .iter()
        .map(|&b| b as isize)
        .collect();

    let mut mach = intcode::Machine::new(springdroid_os, &springdroid_program);
    while let Some(output) = mach.run_until_output() {
        if output < 0x7f {
            if debug {
                print!("{}", output as u8 as char);
            } else if output as u8 == b'D' {
                // 'D' of "Didn't make it across"
                return None;
            }
        } else {
            return Some(output);
        }
    }
    None
}

// super dumb brute force solution
fn _bruteforce_springdroid_program(springdroid_os: &[intcode::Word]) {
    let instr = ["NOT", "AND", "OR"];
    let writables = ["T", "J"];
    let all_regs = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "T", "J"];

    for n in 1..=15 {
        println!("Generating program of length {n}");
        for generated_program in
            iproduct!(instr, all_regs, writables).combinations_with_replacement(n)
        {
            let generated_program: Vec<_> = generated_program
                .iter()
                .map(|(i, r1, r2)| format!("{i} {r1} {r2}"))
                .collect();
            let generated_program: Vec<&str> =
                generated_program.iter().map(|s| s.as_ref()).collect();
            if let Some(output) =
                run_springdroid_program(springdroid_os, &generated_program, "RUN", false)
            {
                println!("Program: {generated_program:?}");
                println!("Hull damage: {}", output);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;

    let springdroid_os = intcode::read_input(input_str.trim());

    // t = !a
    // j = !c
    // j = d & j
    // j = t | j
    // =>
    // j = !a | (d & !c)
    // jump when a is a gap or d is ground and c is a gap
    let walk_springdroid_program = ["NOT C J", "AND D J", "NOT A T", "OR T J"];
    let walk_hull_damage =
        run_springdroid_program(&springdroid_os, &walk_springdroid_program, "WALK", false).unwrap();
    println!("Hull damage with walking: {}", walk_hull_damage);

    // j = d & (!a | (h & (!c | !b)))
    // jump when d is ground and (a is gap or (h is ground and b or c is gap))
    let springdroid_program_extra = [
        "NOT B J", "NOT C T", "OR T J", "AND H J", "NOT A T", "OR T J", "AND D J",
    ];
    let run_hull_damage =
        run_springdroid_program(&springdroid_os, &springdroid_program_extra, "RUN", true).unwrap();
    println!("Hull damage with running: {}", run_hull_damage);

    Ok(())
}
