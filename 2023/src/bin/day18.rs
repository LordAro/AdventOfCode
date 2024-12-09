use std::env;
use std::fs;
use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Coord {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> Vec<((char, usize), usize)> {
    input
        .lines()
        .map(|l| {
            let words: Vec<_> = l.split(' ').collect();
            (
                (words[0].chars().next().unwrap(), words[1].parse().unwrap()),
                usize::from_str_radix(&words[2][2..words[2].len() - 1], 16).unwrap(),
            )
        })
        .collect()
}

// shoelace theorem
fn get_lagoon_size(instrs: &[(char, usize)]) -> i64 {
    let mut points = vec![Coord { x: 0, y: 0 }];
    let mut perimeter_sum: i64 = 0;
    for (instr_dir, instr_count) in instrs {
        let instr_count = *instr_count as i64;
        let pos = points.last().unwrap();
        points.push(match instr_dir {
            'U' => Coord {
                x: pos.x,
                y: pos.y + instr_count,
            },
            'R' => Coord {
                x: pos.x + instr_count,
                y: pos.y,
            },
            'L' => Coord {
                x: pos.x - instr_count,
                y: pos.y,
            },
            'D' => Coord {
                x: pos.x,
                y: pos.y - instr_count,
            },
            _ => unreachable!(),
        });
        perimeter_sum += instr_count;
    }
    // don't need the (duplicated) last coord. Though it doesn't matter as it's just 0,0
    points.pop();

    let area: i64 = (0..points.len())
        .map(|i| {
            (points[i].x * points[(i + 1) % points.len()].y)
                - (points[i].y * points[(i + 1) % points.len()].x)
        })
        .sum();
    (area / 2).abs() + perimeter_sum / 2 + 1
}

fn fix_instructions(instrs: &[((char, usize), usize)]) -> Vec<(char, usize)> {
    instrs
        .iter()
        .map(|(_, colour)| {
            let dir = match colour & 0xf {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => unreachable!(),
            };
            let count = colour >> 4;
            (dir, count)
        })
        .collect()
}

fn main() -> io::Result<()> {
    let instructions = parse_input(&fs::read_to_string(
        env::args().nth(1).expect("Incorrect number of arguments"),
    )?);

    println!(
        "Size of lagoon: {}",
        get_lagoon_size(
            &instructions
                .iter()
                .map(|(dirlen, _)| *dirlen)
                .collect::<Vec<_>>()
        )
    );

    let corrected_instructions = fix_instructions(&instructions);
    println!(
        "True size of lagoon: {}",
        get_lagoon_size(&corrected_instructions)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EX_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn ex1() {
        let instructions: Vec<_> = parse_input(EX_INPUT)
            .iter()
            .map(|(dirlen, _)| *dirlen)
            .collect();
        assert_eq!(get_lagoon_size(&instructions), 62);
    }

    #[test]
    fn ex2() {
        let instructions = parse_input(EX_INPUT);
        let corrected_instructions = fix_instructions(&instructions);
        assert_eq!(get_lagoon_size(&corrected_instructions), 952408144115);
    }
}
