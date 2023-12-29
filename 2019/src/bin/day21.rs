use std::env;
use std::fs;
use std::io;

extern crate advent_of_code;
use advent_of_code::intcode;

extern crate itertools;
use itertools::{iproduct, Itertools};

fn run_springdroid_program(
    springdroid_os: &[intcode::Word],
    instrs: &[&str],
    start_cmd: &str,
) -> Option<isize> {
    let springdroid_program = instrs.join("\n") + "\n" + start_cmd + "\n";
    let springdroid_program: Vec<_> = springdroid_program
        .as_bytes()
        .iter()
        .map(|&b| b as isize)
        .collect();

    let mut mach = intcode::Machine::new(springdroid_os, &springdroid_program);
    while let Some(output) = mach.run_until_output() {
        if output < 0x7f {
            //print!("{}", output as u8 as char);
        } else {
            return Some(output);
        }
    }
    None
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;

    let springdroid_os = intcode::read_input(input_str.trim());

    let walk_springdroid_program = ["NOT A T", "NOT C J", "AND D J", "OR T J"];
    let walk_hull_damage =
        run_springdroid_program(&springdroid_os, &walk_springdroid_program, "WALK").unwrap();
    println!("Hull damage with walking: {}", walk_hull_damage);

    let instr = ["NOT", "AND", "OR"];
    let writables = ["T", "J"];
    let all_regs = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "T", "J"];

    for n in 2..=15 {
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
                run_springdroid_program(&springdroid_os, &generated_program, "RUN")
            {
                println!("Program: {generated_program:?}");
                println!("Hull damage: {}", output);
            }
        }
    }

    Ok(())
}
