use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use advent_of_code::intcode;

type Coord = (usize, usize);

fn get_adjacents(pos: Coord) -> [Coord; 4] {
    [
        (pos.0, pos.1 - 1), // north
        (pos.0, pos.1 + 1), // south
        (pos.0 - 1, pos.1), // west
        (pos.0 + 1, pos.1), // east
    ]
}

const NL: isize = '\n' as isize;
const SCAFFOLD: isize = '#' as isize;
const SPACE: isize = '.' as isize;
const ROBOT: isize = '^' as isize;

fn is_intersection(pos: Coord, scaffolds: &[Vec<isize>]) -> bool {
    get_adjacents(pos)
        .iter()
        .all(|&(x, y)| scaffolds[y][x] == SCAFFOLD)
}

fn parse_scaffolds(mach: &mut intcode::Machine) -> Vec<Vec<isize>> {
    let mut scaffolds = vec![vec![]];
    while let Some(c) = mach.run_until_output() {
        if c == NL {
            if scaffolds.last().unwrap().is_empty() {
                // 2 newlines in a row, we're done
                scaffolds.pop();
                break;
            }
            scaffolds.push(vec![]);
        } else {
            scaffolds.last_mut().unwrap().push(c);
        }
    }
    scaffolds
}

fn get_next_pos(pos: Coord, dir: usize, scaffolds: &[Vec<isize>]) -> Option<Coord> {
    match dir {
        0 => {
            if pos.1 > 0 {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
        1 => {
            if pos.0 < scaffolds[pos.1].len() - 1 {
                Some((pos.0 + 1, pos.1))
            } else {
                None
            }
        }
        2 => {
            if pos.1 < scaffolds.len() - 1 {
                Some((pos.0, pos.1 + 1))
            } else {
                None
            }
        }
        3 => {
            if pos.0 > 0 {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn next_left(pos: Coord, dir: usize, scaffolds: &[Vec<isize>]) -> isize {
    let left_pos = get_next_pos(pos, (dir + 3) % 4, scaffolds);
    if let Some(left_pos) = left_pos {
        scaffolds[left_pos.1][left_pos.0]
    } else {
        SPACE
    }
}

fn next_right(pos: Coord, dir: usize, scaffolds: &[Vec<isize>]) -> isize {
    let right_pos = get_next_pos(pos, (dir + 1) % 4, scaffolds);
    if let Some(right_pos) = right_pos {
        scaffolds[right_pos.1][right_pos.0]
    } else {
        SPACE
    }
}

fn get_ascii_length(seq: &[String]) -> usize {
    seq.iter().map(|s| s.len()).sum::<usize>() + seq.len() - 1
}

fn compress(sequence: &[String]) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    for step in (2..sequence.len() / 3).step_by(2) {
        let func_a = &sequence[0..step];
        if get_ascii_length(func_a) > 20 {
            continue;
        }

        println!("{:?}", func_a);
        for i in (step..sequence.len() - step).step_by(2) {
            let test_case = &sequence[i..i + step];
            if func_a == test_case {
                println!("Found repeat at {}", i);
            }
        }
    }
    (vec![], vec![], vec![], vec![])
}

fn main() {
    let program_str = BufReader::new(
        File::open(
            env::args()
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

    let mut mach = intcode::Machine::new(&program, &[]);
    mach.set_memory(0, 2);
    let scaffolds = parse_scaffolds(&mut mach);

    let mut robot_pos = (0, 0);
    let mut intersections = HashSet::new();
    for y in 1..scaffolds.len() - 1 {
        for x in 1..scaffolds[y].len() - 1 {
            if scaffolds[y][x] == SCAFFOLD && is_intersection((x, y), &scaffolds) {
                intersections.insert((x, y));
            }
            if scaffolds[y][x] == ROBOT {
                robot_pos = (x, y);
            }
        }
    }

    println!(
        "Sum of alignment parameters: {}",
        intersections.iter().map(|&(x, y)| x * y).sum::<usize>()
    );

    let mut seq = vec![];
    let mut robot_dir = 0; // up
    let mut cur_move_count = 0;
    loop {
        let next_pos = get_next_pos(robot_pos, robot_dir, &scaffolds);
        if next_pos.is_none() || scaffolds[next_pos.unwrap().1][next_pos.unwrap().0] == SPACE {
            if cur_move_count != 0 {
                seq.push(cur_move_count.to_string());
            }
            cur_move_count = 0;
            // change dir
            if next_left(robot_pos, robot_dir, &scaffolds) != SPACE {
                seq.push("L".to_string());
                robot_dir = (robot_dir + 3) % 4;
            } else if next_right(robot_pos, robot_dir, &scaffolds) != SPACE {
                seq.push("R".to_string());
                robot_dir = (robot_dir + 1) % 4;
            } else {
                break; // we're done
            }
        } else {
            robot_pos = next_pos.unwrap();
            cur_move_count += 1;
        }
    }
    println!("Generated movement sequence: {:?}", seq);

    let (_main_func, _func_a, _func_b, _func_c) = compress(&seq);
    // XXX Hand rolled!
    let movement_sequence = "A,B,A,B,C,C,B,A,B,C\n\
                             L,8,R,12,R,12,R,10\n\
                             R,10,R,12,R,10\n\
                             L,10,R,10,L,6\n\
                             n\n";
    movement_sequence
        .chars()
        .for_each(|c| mach.push_input(c as isize));

    while let Some(c) = mach.run_until_output() {
        if c > 256 {
            println!("Amount of space dust: {}", c);
        }
    }
}
