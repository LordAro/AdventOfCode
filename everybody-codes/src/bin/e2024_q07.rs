use itertools::Itertools;
use std::collections::btree_set::{BTreeSet, IntoIter};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::io;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Increase,
    Decrease,
    Maintain,
    Start,
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::Increase => '+',
                Action::Decrease => '-',
                Action::Maintain => '=',
                Action::Start => 'S',
            }
        )
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Action, ()> {
        match s {
            "+" => Ok(Action::Increase),
            "-" => Ok(Action::Decrease),
            "=" => Ok(Action::Maintain),
            "S" => Ok(Action::Start),
            _ => Err(()),
        }
    }
}

impl Action {
    fn from(c: char) -> Result<Action, ()> {
        match c {
            '+' => Ok(Action::Increase),
            '-' => Ok(Action::Decrease),
            '=' => Ok(Action::Maintain),
            'S' => Ok(Action::Start),
            _ => Err(()),
        }
    }
}

fn parse_device_actions(input: &str) -> HashMap<char, Vec<Action>> {
    input
        .lines()
        .map(|l| {
            let key = l.chars().next().unwrap();
            let actions: Vec<_> = l[2..].split(',').map(|c| c.parse().unwrap()).collect();
            (key, actions)
        })
        .collect()
}

// Extremely hacky path following. Convert to byte array for indexing purposes
fn parse_track_fancy(input: &[&[u8]]) -> Vec<Action> {
    let mut seen_coords = HashSet::<(usize, usize)>::new();
    let mut coord = (0, 1); // y, x
    let mut parsed_track = vec![Action::from(input[coord.0][coord.1] as char).unwrap()];
    while input[coord.0][coord.1] != b'S' {
        if coord.1 < input[coord.0].len() - 1
            && input[coord.0][coord.1 + 1] != b' '
            && !seen_coords.contains(&(coord.0, coord.1 + 1))
        {
            // E
            coord = (coord.0, coord.1 + 1);
        } else if coord.0 < input.len() - 1
            && input[coord.0 + 1][coord.1] != b' '
            && !seen_coords.contains(&(coord.0 + 1, coord.1))
        {
            // S
            coord = (coord.0 + 1, coord.1);
        } else if coord.1 > 0
            && input[coord.0][coord.1 - 1] != b' '
            && !seen_coords.contains(&(coord.0, coord.1 - 1))
        {
            // W
            coord = (coord.0, coord.1 - 1);
        } else if coord.0 > 0
            && input[coord.0 - 1][coord.1] != b' '
            && !seen_coords.contains(&(coord.0 - 1, coord.1))
        {
            // N
            coord = (coord.0 - 1, coord.1);
        } else {
            unreachable!();
        }
        seen_coords.insert(coord);
        parsed_track.push(Action::from(input[coord.0][coord.1] as char).unwrap());
    }
    parsed_track
}

fn run_race_individual(actions: &[Action], total_rounds: usize) -> usize {
    actions
        .iter()
        .cycle()
        .take(total_rounds)
        .scan(10, |cur_power, action| {
            match action {
                Action::Increase => *cur_power += 1,
                Action::Decrease => *cur_power -= 1,
                _ => (),
            };
            Some(*cur_power)
        })
        .sum()
}

fn run_race(
    device_actions: &HashMap<char, Vec<Action>>,
    total_rounds: usize,
) -> HashMap<char, usize> {
    device_actions
        .iter()
        .map(|(&key, action_plan)| (key, run_race_individual(action_plan, total_rounds)))
        .collect()
}

fn run_race_track_individual(track: &[Action], actions: &[Action], lap_count: usize) -> usize {
    track
        .iter()
        .cycle()
        .zip(actions.iter().cycle())
        .take(track.len() * lap_count)
        .scan(10, |cur_power, (track_action, action)| {
            match (track_action, action) {
                (Action::Increase, _) => *cur_power += 1,
                (Action::Decrease, _) => *cur_power -= 1,
                // order important
                (_, Action::Increase) => *cur_power += 1,
                (_, Action::Decrease) => *cur_power -= 1,
                _ => (),
            };
            Some(*cur_power)
        })
        .sum()
}

fn run_track(
    track: &[Action],
    device_actions: &HashMap<char, Vec<Action>>,
    lap_count: usize,
) -> HashMap<char, usize> {
    device_actions
        .iter()
        .map(|(&key, action_plan)| {
            (
                key,
                run_race_track_individual(track, action_plan, lap_count),
            )
        })
        .collect()
}

fn get_race_results(results: &HashMap<char, usize>) -> String {
    results
        .iter()
        .sorted_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .rev()
        .collect()
}

// Taken from https://stackoverflow.com/questions/59939808/how-to-iterate-over-all-unique-permutations-of-a-sequence-in-rust
enum UniquePermutations<T> {
    Leaf {
        elements: Option<Vec<T>>,
    },
    Stem {
        elements: Vec<T>,
        unique_elements: IntoIter<T>,
        first_element: T,
        inner: Box<Self>,
    },
}

impl<T: Clone + Ord> UniquePermutations<T> {
    fn new(elements: Vec<T>) -> Self {
        if elements.len() == 1 {
            let elements = Some(elements);
            Self::Leaf { elements }
        } else {
            let mut unique_elements = elements
                .clone()
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter();

            let (first_element, inner) = Self::next_level(&mut unique_elements, elements.clone())
                .expect("Must have at least one item");

            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            }
        }
    }

    fn next_level(
        mut unique_elements: impl Iterator<Item = T>,
        elements: Vec<T>,
    ) -> Option<(T, Box<Self>)> {
        let first_element = unique_elements.next()?;

        let mut remaining_elements = elements;

        if let Some(idx) = remaining_elements.iter().position(|i| *i == first_element) {
            remaining_elements.remove(idx);
        }

        let inner = Box::new(Self::new(remaining_elements));

        Some((first_element, inner))
    }
}

impl<T: Clone + Ord> Iterator for UniquePermutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Leaf { elements } => elements.take(),
            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            } => loop {
                if let Some(mut v) = inner.next() {
                    v.insert(0, first_element.clone());
                    return Some(v);
                }
                let (next_fe, next_i) = Self::next_level(&mut *unique_elements, elements.clone())?;
                *first_element = next_fe;
                *inner = next_i;
            },
        }
    }
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    {
        let device_actions = parse_device_actions(&fs::read_to_string(p1_input_filename)?);
        let race_results = get_race_results(&run_race(&device_actions, 10));
        println!("P1: Race results: {race_results}");
    }

    {
        let race_track = parse_track_fancy(&[
            "S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--".as_bytes(),
            "-                                                                     -".as_bytes(),
            "=                                                                     =".as_bytes(),
            "+                                                                     +".as_bytes(),
            "=                                                                     +".as_bytes(),
            "+                                                                     =".as_bytes(),
            "=                                                                     =".as_bytes(),
            "-                                                                     -".as_bytes(),
            "--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-".as_bytes(),
        ]);

        let device_actions = parse_device_actions(&fs::read_to_string(p2_input_filename)?);
        let race_results = get_race_results(&run_track(&race_track, &device_actions, 10));
        println!("P2: Race results: {race_results}");
    }

    {
        let race_track = parse_track_fancy(&[
            "S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--".as_bytes(),
            "- + +   + =   =     =      =   == = - -     - =  =         =-=        -".as_bytes(),
            "= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++".as_bytes(),
            "+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=       ".as_bytes(),
            "= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =          ".as_bytes(),
            "+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==".as_bytes(),
            "=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =".as_bytes(),
            "-               = = = =   +  +  ==+ = = +   =        ++    =          -".as_bytes(),
            "-               = + + =   +  -  = + = = +   =        +     =          -".as_bytes(),
            "--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-".as_bytes(),
        ]);
        let device_actions = parse_device_actions(&fs::read_to_string(p3_input_filename)?);
        // The full race is 2024 laps, but a cycle forms at 11 laps so if we're ahead then we're
        // going to stay ahead
        let num_laps = 11;
        let opposition_score =
            run_race_track_individual(&race_track, &device_actions[&'A'], num_laps);

        let initial_action_plan_str = "+++++---===";
        let initial_action_plan: Vec<_> = initial_action_plan_str
            .chars()
            .map(|c| Action::from(c).unwrap())
            .collect();
        let num_winning_plans = UniquePermutations::new(initial_action_plan)
            .filter(|action_plan| {
                let score = run_race_track_individual(&race_track, action_plan, num_laps);
                score > opposition_score
            })
            .count();
        println!(
            "P3: Number of winning action plans: {num_winning_plans} (beating {opposition_score})"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn parse_track_hardcoded(input: &str) -> Vec<Action> {
        input.chars().map(|c| Action::from(c).unwrap()).collect()
    }

    #[test]
    fn ex1() {
        let input_str = "A:+,-,=,=\nB:+,=,-,+\nC:=,-,+,+\nD:=,=,=,+";

        let actions = parse_device_actions(input_str);
        let race_scores = run_race(&actions, 10);
        let expected_scores = HashMap::from([('A', 103), ('B', 116), ('C', 107), ('D', 110)]);
        assert_eq!(race_scores, expected_scores);
        let race_result = get_race_results(&race_scores);
        assert_eq!(race_result, "BDCA");
    }

    #[test]
    fn ex2() {
        let input_str = "A:+,-,=,=\nB:+,=,-,+\nC:=,-,+,+\nD:=,=,=,+";
        let race_track = parse_track_hardcoded("+===++-=+=-S");

        let actions = parse_device_actions(input_str);
        let race_result = get_race_results(&run_track(&race_track, &actions, 10));
        assert_eq!(race_result, "DCBA");
    }

    #[test]
    fn ex2_track() {
        let expected_track = parse_track_hardcoded("+===++-=+=-S");
        let input_track = ["S+===".as_bytes(), "-   +".as_bytes(), "=+=-+".as_bytes()];
        assert_eq!(parse_track_fancy(&input_track), expected_track);
    }
}
