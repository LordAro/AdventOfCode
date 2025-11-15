use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

struct State {
    dragon: Coord,
    limit: Coord,
    sheep: HashSet<Coord>,
    hideouts: HashSet<Coord>,
}

fn parse_grid(input: &str) -> State {
    let mut dragon = Coord { x: 0, y: 0 };
    let mut sheep = HashSet::new();
    let mut hideouts = HashSet::new();
    let mut limit = Coord { x: 0, y: 0 };
    for (y, row) in input.lines().enumerate() {
        limit.y = limit.y.max(y);
        for (x, cell) in row.chars().enumerate() {
            limit.x = limit.x.max(x);
            match cell {
                'S' => _ = sheep.insert(Coord { x, y }),
                'D' => dragon = Coord { x, y },
                '#' => _ = hideouts.insert(Coord { x, y }),
                _ => {}
            }
        }
    }
    State {
        dragon,
        limit,
        sheep,
        hideouts,
    }
}

fn knight_move(p: Coord, limit: Coord) -> HashSet<Coord> {
    let mut moves = HashSet::new();
    if p.y > 1 {
        if p.x > 0 {
            moves.insert(Coord {
                x: p.x - 1,
                y: p.y - 2,
            });
        }
        if p.x < limit.x {
            moves.insert(Coord {
                x: p.x + 1,
                y: p.y - 2,
            });
        }
    }
    if p.x > 1 {
        if p.y > 0 {
            moves.insert(Coord {
                x: p.x - 2,
                y: p.y - 1,
            });
        }
        if p.y < limit.y {
            moves.insert(Coord {
                x: p.x - 2,
                y: p.y + 1,
            });
        }
    }
    if p.x < limit.x - 1 {
        if p.y > 0 {
            moves.insert(Coord {
                x: p.x + 2,
                y: p.y - 1,
            });
        }
        if p.y < limit.y {
            moves.insert(Coord {
                x: p.x + 2,
                y: p.y + 1,
            });
        }
    }
    if p.y < limit.y - 1 {
        if p.x > 0 {
            moves.insert(Coord {
                x: p.x - 1,
                y: p.y + 2,
            });
        }
        if p.x < limit.x {
            moves.insert(Coord {
                x: p.x + 1,
                y: p.y + 2,
            });
        }
    }
    moves
}

fn get_dragon_positions(dragon: Coord, limit: Coord, num_moves: usize) -> HashSet<Coord> {
    let mut total_positions = HashSet::new();
    total_positions.insert(dragon);
    for _ in 0..num_moves {
        let new_positions: HashSet<Coord> = total_positions
            .iter()
            .flat_map(|p| knight_move(*p, limit))
            .collect();
        total_positions.extend(new_positions);
    }
    total_positions
}

fn num_sheep_in_range_after_n(state: &State, num_moves: usize) -> usize {
    let dragons = get_dragon_positions(state.dragon, state.limit, num_moves);
    state.sheep.intersection(&dragons).count()
}

fn num_sheep_in_range_with_movement(state: &State, num_moves: usize) -> usize {
    let mut num_eaten = 0;
    let mut dragons = HashSet::from([state.dragon]);
    let mut remaining_sheep = state.sheep.clone();
    for _ in 0..num_moves {
        let new_dragon_positions: HashSet<Coord> = dragons
            .iter()
            .flat_map(|p| knight_move(*p, state.limit))
            .collect();
        dragons = new_dragon_positions;
        let non_hidden_dragons: HashSet<_> = dragons.difference(&state.hideouts).collect();

        let num_sheep = remaining_sheep.len();
        remaining_sheep.retain(|s| !non_hidden_dragons.contains(s));
        num_eaten += num_sheep - remaining_sheep.len();

        // sheep move
        // oops, sheep moved into a dragon
        let new_sheep_positions: HashSet<_> = remaining_sheep
            .iter()
            .map(|s| Coord { x: s.x, y: s.y + 1 })
            .collect();
        remaining_sheep = new_sheep_positions;
        let num_sheep = remaining_sheep.len();
        remaining_sheep.retain(|s| !non_hidden_dragons.contains(s));
        num_eaten += num_sheep - remaining_sheep.len();
    }
    num_eaten
}

fn count_unique_sequences(state: State, cache: &mut HashMap<(Coord, Vec<Coord>), usize>) -> usize {
    // sheep escaped, not a winning game
    if state.sheep.iter().any(|s| s.y > state.limit.y) {
        return 0;
    }

    // all sheep eaten, we win!
    if state.sheep.is_empty() {
        return 1;
    }

    let state_key = (state.dragon, state.sheep.clone().into_iter().collect());
    if let Some(seqs) = cache.get(&state_key) {
        return *seqs;
    }

    let mut total_seqs = 0;
    let mut has_sheep_moved = false;
    for sheep in &state.sheep {
        // if sheep can move, it must
        let new_sheep = Coord {
            x: sheep.x,
            y: sheep.y + 1,
        };
        if new_sheep == state.dragon && !state.hideouts.contains(&new_sheep) {
            // sheep don't move into a dragon
            continue;
        }
        has_sheep_moved = true;
        let mut new_sheeps = state.sheep.clone();
        new_sheeps.remove(sheep);
        new_sheeps.insert(new_sheep);

        for new_dragon in knight_move(state.dragon, state.limit) {
            let mut sheep_after_dragon = new_sheeps.clone();
            if new_sheeps.contains(&new_dragon) && !state.hideouts.contains(&new_dragon) {
                sheep_after_dragon.remove(&new_dragon);
            }
            let new_state = State {
                dragon: new_dragon,
                limit: state.limit,
                sheep: sheep_after_dragon,
                hideouts: state.hideouts.clone(),
            };
            total_seqs += count_unique_sequences(new_state, cache);
        }
    }

    if !has_sheep_moved {
        for new_dragon in knight_move(state.dragon, state.limit) {
            let mut sheep_after_dragon = state.sheep.clone();
            if sheep_after_dragon.contains(&new_dragon) && !state.hideouts.contains(&new_dragon) {
                sheep_after_dragon.remove(&new_dragon);
            }
            let new_state = State {
                dragon: new_dragon,
                limit: state.limit,
                sheep: sheep_after_dragon,
                hideouts: state.hideouts.clone(),
            };
            total_seqs += count_unique_sequences(new_state, cache);
        }
    }
    cache.insert(state_key, total_seqs);
    total_seqs
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_state = parse_grid(&fs::read_to_string(p1_input_filename)?);
    let p1_num_sheep = num_sheep_in_range_after_n(&p1_state, 4);

    let p2_state = parse_grid(&fs::read_to_string(p2_input_filename)?);
    let p2_num_eaten = num_sheep_in_range_with_movement(&p2_state, 20);

    let p3_state = parse_grid(&fs::read_to_string(p3_input_filename)?);
    let mut p3_cache = HashMap::new();
    let p3_move_sequence_count = count_unique_sequences(p3_state, &mut p3_cache);

    println!("P1: Number of sheep coverable in 4 moves: {p1_num_sheep}");
    println!("P2: Number of sheep eaten after 20 moves: {p2_num_eaten}");
    println!("P3: Number of unique move sequences: {p3_move_sequence_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = "...SSS.......
.S......S.SS.
..S....S...S.
..........SS.
..SSSS...S...
.....SS..S..S
SS....D.S....
S.S..S..S....
....S.......S
.SSS..SS.....
.........S...
.......S....S
SS.....S..S..";

        let state = parse_grid(input);
        assert_eq!(num_sheep_in_range_after_n(&state, 3), 27);
    }

    #[test]
    fn ex3a() {
        let input = "SSS
..#
#.#
#D.";
        let state = parse_grid(input);
        let mut cache = HashMap::new();
        assert_eq!(count_unique_sequences(state, &mut cache), 15);
    }

    #[test]
    fn ex3b() {
        let input = "SSS
..#
..#
.##
.D#";
        let state = parse_grid(input);
        let mut cache = HashMap::new();
        assert_eq!(count_unique_sequences(state, &mut cache), 8);
    }

    #[test]
    fn ex3c() {
        let input = "..S..
.....
..#..
.....
..D..";
        let state = parse_grid(input);
        let mut cache = HashMap::new();
        assert_eq!(count_unique_sequences(state, &mut cache), 44);
    }
}
