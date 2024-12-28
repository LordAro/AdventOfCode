use itertools::Itertools;
use std::env;
use std::fs;
use std::io;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Coord {
    fn to_movement_str(self) -> String {
        let x_char = if self.x < 0 { "<" } else { ">" };
        let y_char = if self.y < 0 { "v" } else { "^" };
        x_char.repeat(self.x.unsigned_abs()) + &y_char.repeat(self.y.unsigned_abs())
    }
}

fn keypad_coords(c: char) -> Coord {
    // 789
    // 456
    // 123
    //  0A
    match c {
        '7' => Coord { x: 0, y: 3 },
        '8' => Coord { x: 1, y: 3 },
        '9' => Coord { x: 2, y: 3 },
        '4' => Coord { x: 0, y: 2 },
        '5' => Coord { x: 1, y: 2 },
        '6' => Coord { x: 2, y: 2 },
        '1' => Coord { x: 0, y: 1 },
        '2' => Coord { x: 1, y: 1 },
        '3' => Coord { x: 2, y: 1 },
        // blank
        '0' => Coord { x: 1, y: 0 },
        'A' => Coord { x: 2, y: 0 },
        _ => unreachable!(),
    }
}

fn get_keypad_vectors(code: &str) -> Vec<String> {
    let mut cur_pos = 'A';
    let mut overall_movement = vec![];
    for c in code.chars() {
        let movement = keypad_coords(c) - keypad_coords(cur_pos);
        let move_str = movement.to_movement_str();
        if !move_str.is_empty() {
            overall_movement.push(move_str);
        }
        overall_movement.push("A".to_string());
        cur_pos = c;
    }
    overall_movement
}

fn directional_coords(c: char) -> Coord {
    //  ^A
    // <v>
    match c {
        // blank
        '^' => Coord { x: 1, y: 1 },
        'A' => Coord { x: 2, y: 1 },
        '<' => Coord { x: 0, y: 0 },
        'v' => Coord { x: 1, y: 0 },
        '>' => Coord { x: 2, y: 0 },
        _ => unreachable!(),
    }
}

fn get_directional_moves(code: &str) -> String {
    let mut cur_pos = 'A';
    let mut overall_movement = "".to_string();
    for c in code.chars() {
        let movement = directional_coords(c) - directional_coords(cur_pos);
        overall_movement += &movement.to_movement_str();
        overall_movement += "A";
        cur_pos = c;
    }
    overall_movement
}

fn get_directional_vectors(code: &str) -> Vec<String> {
    let mut cur_pos = 'A';
    let mut overall_movement = vec![];
    for c in code.chars() {
        let movement = directional_coords(c) - directional_coords(cur_pos);
        let move_str = movement.to_movement_str();
        if !move_str.is_empty() {
            overall_movement.push(move_str);
        }
        overall_movement.push("A".to_string());
        cur_pos = c;
    }
    overall_movement
}

fn route_goes_over_blank(code: &str, blank: Coord) -> bool {
    let mut cur_pos = keypad_coords('A');
    for c in code.chars() {
        match c {
            '^' => cur_pos.y += 1,
            'v' => cur_pos.y -= 1,
            '>' => cur_pos.x += 1,
            '<' => cur_pos.x -= 1,
            _ => (),
        }
        if cur_pos == blank {
            return true;
        }
    }
    false
}

// fun with iterators.
// all permutations of inner elements while maintaining existing order of outer elements
fn all_inner_combinations(input: &[String]) -> impl Iterator<Item = String> + use<'_> {
    input
        .iter()
        .map(|c| c.chars().permutations(c.len()))
        .multi_cartesian_product()
        .map(|v| v.iter().flat_map(|v2| v2.iter()).collect())
}

fn get_shortest_derivative_directional_moves(code: &str) -> String {
    all_inner_combinations(&get_keypad_vectors(code))
        .filter(|kv| !route_goes_over_blank(kv, Coord { x: 0, y: 0 }))
        .flat_map(|kv| {
            all_inner_combinations(&get_directional_vectors(&kv))
                .filter(|kv| !route_goes_over_blank(kv, Coord { x: 0, y: 1 }))
                .map(|dv| get_directional_moves(&dv))
                .collect::<Vec<_>>()
        })
        .min_by_key(|dv| dv.len())
        .unwrap()
}

fn main() -> io::Result<()> {
    let codes: Vec<String> = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
        .lines()
        .map(|s| s.to_string())
        .collect();

    let code_complexity: usize = codes
        .iter()
        .map(|code| {
            let moves = get_shortest_derivative_directional_moves(code);
            let num: usize = code[0..3].parse().unwrap();
            moves.len() * num
        })
        .sum();
    println!("P1: Code complexity score: {code_complexity}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1a() {
        let code = "029A";
        let moves = get_shortest_derivative_directional_moves(code);
        assert_eq!(
            moves.len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn ex1b() {
        let code = "980A";
        let moves = get_shortest_derivative_directional_moves(code);
        assert_eq!(
            moves.len(),
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
        );
    }

    #[test]
    fn ex1c() {
        let code = "179A";
        let moves = get_shortest_derivative_directional_moves(code);
        assert_eq!(
            moves.len(),
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn ex1d() {
        let code = "456A";
        let moves = get_shortest_derivative_directional_moves(code);
        assert_eq!(
            moves.len(),
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
        );
    }

    #[test]
    fn ex1e() {
        let code = "379A";
        let moves = get_shortest_derivative_directional_moves(code);
        assert_eq!(
            moves.len(),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }
}
