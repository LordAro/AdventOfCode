use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Space,
    Box,
    LeftBox,
    RightBox,
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
                    '[' => Cell::LeftBox,
                    ']' => Cell::RightBox,
                    '#' => Cell::Wall,
                    '@' => Cell::Robot,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn expand_grid(initial_grid: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    initial_grid
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|c| match c {
                    Cell::Space => [Cell::Space, Cell::Space],
                    Cell::Box => [Cell::LeftBox, Cell::RightBox],
                    Cell::Wall => [Cell::Wall, Cell::Wall],
                    Cell::Robot => [Cell::Robot, Cell::Space],
                    Cell::LeftBox | Cell::RightBox => unreachable!(),
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

    let mut coords_to_move = vec![HashSet::from([*robot_pos])];

    while !coords_to_move.last().unwrap().is_empty() {
        // Use a hashset to prevent duplicates (e.g. both left and right adding left and right each)
        let mut try_move = HashSet::default();
        for coord in coords_to_move.last().unwrap() {
            let target_coord = match instr {
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

            match grid[target_coord.y][target_coord.x] {
                // Can't move, return early
                Cell::Wall => return,
                Cell::Box => _ = try_move.insert(target_coord),
                Cell::LeftBox => {
                    try_move.insert(target_coord);
                    if instr == '^' || instr == 'v' {
                        try_move.insert(Coord {
                            x: target_coord.x + 1,
                            y: target_coord.y,
                        });
                    }
                }
                Cell::RightBox => {
                    try_move.insert(target_coord);
                    if instr == '^' || instr == 'v' {
                        try_move.insert(Coord {
                            x: target_coord.x - 1,
                            y: target_coord.y,
                        });
                    }
                }
                Cell::Space => (),
                Cell::Robot => unreachable!(), // robot never tries to push itself
            }
        }
        coords_to_move.push(try_move);
    }

    for line in coords_to_move.iter().rev() {
        for coord in line {
            let new_coord = match instr {
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

            assert!(matches!(
                grid[coord.y][coord.x],
                Cell::Robot | Cell::Box | Cell::LeftBox | Cell::RightBox
            ));
            grid[new_coord.y][new_coord.x] = grid[coord.y][coord.x];
            grid[coord.y][coord.x] = Cell::Space;
            if grid[new_coord.y][new_coord.x] == Cell::Robot {
                *robot_pos = new_coord;
            }
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<Cell>]) {
    for row in grid {
        for c in row {
            print!(
                "{}",
                match c {
                    Cell::Wall => '#',
                    Cell::Robot => '@',
                    Cell::Space => '.',
                    Cell::Box => 'O',
                    Cell::LeftBox => '[',
                    Cell::RightBox => ']',
                }
            );
        }
        println!();
    }
}

fn find_robot(grid: &[Vec<Cell>]) -> Coord {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            let x = row.iter().position(|c| *c == Cell::Robot)?;
            Some(Coord { x, y })
        })
        .unwrap()
}

fn grid_gps_sum(grid: &[Vec<Cell>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| match c {
                    Cell::Box | Cell::LeftBox => 100 * y + x,
                    _ => 0,
                })
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
    let mut expanded_grid = expand_grid(&grid);
    let mut robot_pos = find_robot(&grid);

    for instr in move_instrs.chars() {
        do_move(&mut grid, &mut robot_pos, instr);
    }
    println!("P1: Box GPS coordinate sum: {}", grid_gps_sum(&grid));

    let mut robot_pos = find_robot(&expanded_grid);
    for instr in move_instrs.chars() {
        do_move(&mut expanded_grid, &mut robot_pos, instr);
    }
    println!(
        "P2: Wide box GPS coordinate sum: {}",
        grid_gps_sum(&expanded_grid)
    );

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

    #[test]
    fn ex2() {
        let input_grid = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";
        let input_instr = "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let mut grid = expand_grid(&parse_grid(&input_grid));
        let mut robot_pos = find_robot(&grid);
        for instr in input_instr.chars() {
            println!("{}", instr);
            do_move(&mut grid, &mut robot_pos, instr);
            print_grid(&grid);
        }
        assert_eq!(grid_gps_sum(&grid), 9021);
    }
}
