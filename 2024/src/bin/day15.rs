use std::env;
use std::fs;
use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq)]
enum Cell {
    Space,
    Box,
    Robot,
    Wall,
}

fn parse_grid(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Cell::Space,
                    'O' => Cell::Box,
                    '#' => Cell::Wall,
                    '@' => Cell::Robot,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn do_move(grid: &mut [Vec<Cell>], robot_pos: &mut Coord, instr: char) {
    // instr string is broken up into various newlines
    if instr == '\n' {
        return;
    }
    assert!(grid[robot_pos.y][robot_pos.x] == Cell::Robot);

    let mut move_list = vec![];
    let mut coord = *robot_pos;
    while grid[coord.y][coord.x] != Cell::Space {
        if grid[coord.y][coord.x] == Cell::Wall {
            // Can't move, return early
            return;
        }

        move_list.push(coord);

        coord = match instr {
            '^' => Coord {
                x: coord.x,
                y: coord.y - 1,
            },
            '>' => Coord {
                x: coord.x + 1,
                y: coord.y,
            },
            'v' => Coord {
                x: coord.x,
                y: coord.y + 1,
            },
            '<' => Coord {
                x: coord.x - 1,
                y: coord.y,
            },
            _ => unreachable!("Got unexpected direction: {}", instr as u8),
        };
    }
    assert!(grid[coord.y][coord.x] == Cell::Space);

    grid[robot_pos.y][robot_pos.x] = Cell::Space;
    if move_list.len() == 1 {
        grid[coord.y][coord.x] = Cell::Robot;
        *robot_pos = coord;
    } else {
        grid[coord.y][coord.x] = Cell::Box;
        let new_robot_pos = move_list[1];
        grid[new_robot_pos.y][new_robot_pos.x] = Cell::Robot;
        *robot_pos = new_robot_pos;
    }
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<Cell>]) {
    for row in grid {
        for c in row {
            print!("{}", match c {
                Cell::Wall => '#',
                Cell::Robot => '@',
                Cell::Space => '.',
                Cell::Box => 'O',
            });
        }
        println!();
    }
}

fn find_robot(grid: &[Vec<Cell>]) -> Coord {
    grid.iter().enumerate().find_map(|(y, row)| {
        let x = row.iter().position(|c| *c == Cell::Robot)?;
        Some(Coord { x, y })
    }).unwrap()
}

fn grid_gps_sum(grid: &[Vec<Cell>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == Cell::Box)
                .map(|(x, _)| 100 * y + x)
                .sum::<usize>()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;
    let Some((grid_str, move_instrs)) = input.split_once("\n\n") else {
        unreachable!()
    };
    let mut grid = parse_grid(grid_str);
    let mut robot_pos = find_robot(&grid);

    for instr in move_instrs.chars() {
        do_move(&mut grid, &mut robot_pos, instr);
    }
    println!("P1: Box GPS coordinate sum: {}", grid_gps_sum(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input_grid = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        let input_instr = "<^^>>>vv<v>>v<<";
        let mut grid = parse_grid(&input_grid);
        let mut robot_pos = find_robot(&grid);
        for instr in input_instr.chars() {
            do_move(&mut grid, &mut robot_pos, instr);
            print_grid(&grid);
        }
        assert_eq!(grid_gps_sum(&grid), 2028);
    }
}
