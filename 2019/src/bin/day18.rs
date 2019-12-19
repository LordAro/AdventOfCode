use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (usize, usize);

#[derive(Debug, PartialEq)]
enum State {
    Clear,
    Wall,
    Key(char),
    Door(char),
    Me,
}

fn parse_map<T: AsRef<str>>(
    input_lines: &[T],
) -> (
    Vec<Vec<State>>,
    HashMap<Coord, char>,
    HashMap<Coord, char>,
    Coord,
) {
    let mut keys: HashMap<Coord, char> = HashMap::new();
    let mut doors: HashMap<Coord, char> = HashMap::new();
    let mut me: Coord = Default::default();
    let map = input_lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.as_ref()
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => State::Wall,
                    '.' => State::Clear,
                    '@' => {
                        me = (x, y);
                        State::Me
                    }
                    'a'..='z' => {
                        keys.insert((x, y), c);
                        State::Key(c)
                    }
                    'A'..='Z' => {
                        doors.insert((x, y), c);
                        State::Door(c)
                    }
                    _ => panic!("Unrecognised character {}", c),
                })
                .collect()
        })
        .collect();
    (map, keys, doors, me)
}

fn open_adjacents(pos: Coord, map: &Vec<Vec<State>>) -> Vec<Coord> {
    let mut ret = vec![];
    if pos.1 > 0 && map[pos.1 - 1][pos.0] != State::Wall {
        ret.push((pos.0, pos.1 - 1));
    }
    if pos.1 < map.len() - 1 && map[pos.1 + 1][pos.0] != State::Wall {
        ret.push((pos.0, pos.1 + 1));
    }
    if pos.0 > 0 && map[pos.1][pos.0 - 1] != State::Wall {
        ret.push((pos.0 - 1, pos.1));
    }
    if pos.0 < map[pos.1].len() - 1 && map[pos.1][pos.0 + 1] != State::Wall {
        ret.push((pos.0 + 1, pos.1));
    }
    ret
}

fn print_map(positions: &Vec<Vec<State>>) {
    for j in 0..positions.len() {
        for i in 0..positions[j].len() {
            match positions[j][i] {
                State::Clear => print!("."),
                State::Wall => print!("#"),
                State::Me => print!("@"),
                State::Key(k) => print!("{}", k),
                State::Door(d) => print!("{}", d),
            }
        }
        println!();
    }
}

fn get_route(source: Coord, target: Coord, map: &Vec<Vec<State>>) -> Vec<Coord> {
    let mut came_from: HashMap<Coord, Coord> = HashMap::new();
    let mut open_set = VecDeque::new();
    open_set.push_back(source);

    let mut g_score = HashMap::new();
    g_score.insert(source, 0);

    while !open_set.is_empty() {
        let current = open_set.pop_front().unwrap();
        if current == target {
            let mut total_path = vec![current];
            let mut current = current;
            while came_from.contains_key(&current) {
                current = came_from[&current];
                total_path.push(current);
            }
            total_path.reverse();
            return total_path;
        }

        for adj in open_adjacents(current, map).into_iter() {
            let dist = g_score[&current] + 1;
            if &dist < g_score.get(&adj).unwrap_or(&isize::max_value()) {
                came_from.insert(adj, current);
                g_score.insert(adj, dist);
                open_set.push_back(adj);
            }
        }
    }
    panic!("Unable to find route between {:?} and {:?}", source, target);
}

fn tsp(
    start: Coord,
    remaining_keys: &HashMap<Coord, char>,
    keys_so_far: &HashSet<char>,
    doors: &HashMap<Coord, char>,
    map: &Vec<Vec<State>>,
    route_cache: &HashMap<(Coord, Coord), Vec<Coord>>,
) -> impl Iterator<Item = Coord> {
    let mut shortest_route = vec![];
    let mut shortest_dist = usize::max_value();
    for (&key, key_name) in remaining_keys {
        let mut route = route_cache.get(&(start, key)).unwrap().clone();
        if route
            .iter()
            .filter_map(|c| {
                if doors.contains_key(&c) {
                    Some(doors[c])
                } else {
                    None
                }
            })
            .any(|d| !keys_so_far.contains(&d.to_ascii_lowercase()))
        {
            continue;
        }
        //println!("{:?} -> {:?} = {}", start, key, route.len() - 1);
        let mut new_remaining_keys = remaining_keys.clone();
        new_remaining_keys.remove(&key);
        let mut new_keys_so_far = keys_so_far.clone();
        new_keys_so_far.insert(*key_name);
        let recurse = tsp(
            key,
            &new_remaining_keys,
            &new_keys_so_far,
            doors,
            map,
            route_cache,
        );
        route.extend(recurse);
        if route.len() < shortest_dist {
            shortest_dist = route.len();
            shortest_route = route;
        }
    }
    //shortest_dist - 1
    shortest_route.into_iter().skip(1) // Remove starting position from the route
}

fn build_route_cache(
    keys: &HashMap<Coord, char>,
    start: Coord,
    map: &Vec<Vec<State>>,
) -> HashMap<(Coord, Coord), Vec<Coord>> {
    let mut route_cache: HashMap<(Coord, Coord), Vec<Coord>> = HashMap::new();
    for &key1 in keys.keys() {
        let route = get_route(start, key1, &map);
        route_cache.insert((start, key1), route);
        for &key2 in keys.keys() {
            if key1 == key2 {
                continue;
            }
            let route = get_route(key1, key2, &map);
            route_cache.insert((key1, key2), route);
        }
    }
    route_cache
}

fn main() {
    let input_lines: Vec<_> = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| l.unwrap())
    .collect();

    let (map, keys, doors, me) = parse_map(&input_lines);
    print_map(&map);

    let route_cache = build_route_cache(&keys, me, &map);

    let final_route: Vec<_> = tsp(me, &keys, &HashSet::new(), &doors, &map, &route_cache).collect();
    println!("Route length: {} {:?}", final_route.len(), final_route);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input: Vec<_> = "#########\n\
                             #b.A.@.a#\n\
                             #########"
            .lines()
            .collect();
        let (map, keys, doors, me) = parse_map(&input);
        let route_cache = build_route_cache(&keys, me, &map);
        let final_route: Vec<_> =
            tsp(me, &keys, &HashSet::new(), &doors, &map, &route_cache).collect();
        assert_eq!(final_route.len(), 8);
    }

    #[test]
    fn ex2() {
        let input: Vec<_> = "########################\n\
                             #f.D.E.e.C.b.A.@.a.B.c.#\n\
                             ######################.#\n\
                             #d.....................#\n\
                             ########################"
            .lines()
            .collect();
        let (map, keys, doors, me) = parse_map(&input);
        let route_cache = build_route_cache(&keys, me, &map);
        let final_route: Vec<_> =
            tsp(me, &keys, &HashSet::new(), &doors, &map, &route_cache).collect();
        assert_eq!(final_route.len(), 86);
    }
    #[test]
    fn ex3() {
        let input: Vec<_> = "########################\n\
                             #...............b.C.D.f#\n\
                             #.######################\n\
                             #.....@.a.B.c.d.A.e.F.g#\n\
                             ########################"
            .lines()
            .collect();
        let (map, keys, doors, me) = parse_map(&input);
        let route_cache = build_route_cache(&keys, me, &map);
        let final_route: Vec<_> =
            tsp(me, &keys, &HashSet::new(), &doors, &map, &route_cache).collect();
        assert_eq!(final_route.len(), 132);
    }

    #[test]
    fn ex4() {
        let input: Vec<_> = "#################\n\
                             #i.G..c...e..H.p#\n\
                             ########.########\n\
                             #j.A..b...f..D.o#\n\
                             ########@########\n\
                             #k.E..a...g..B.n#\n\
                             ########.########\n\
                             #l.F..d...h..C.m#\n\
                             #################"
            .lines()
            .collect();
        let (map, keys, doors, me) = parse_map(&input);
        let route_cache = build_route_cache(&keys, me, &map);
        let final_route: Vec<_> =
            tsp(me, &keys, &HashSet::new(), &doors, &map, &route_cache).collect();
        assert_eq!(final_route.len(), 136);
    }

    #[test]
    fn ex5() {
        let input: Vec<_> = "########################\n\
                             #@..............ac.GI.b#\n\
                             ###d#e#f################\n\
                             ###A#B#C################\n\
                             ###g#h#i################\n\
                             ########################"
            .lines()
            .collect();
        let (map, keys, doors, me) = parse_map(&input);
        let route_cache = build_route_cache(&keys, me, &map);
        let final_route: Vec<_> =
            tsp(me, &keys, &HashSet::new(), &doors, &map, &route_cache).collect();
        assert_eq!(final_route.len(), 81);
    }
}
