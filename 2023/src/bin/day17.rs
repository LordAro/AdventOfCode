use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

fn get_adjacents(map: &Vec<Vec<u32>>, pos: Coord) -> [Option<Coord>; 4] {
    [
        pos.y.checked_sub(1).map(|y| Coord { x: pos.x, y }), // north
        Some(Coord {
            x: pos.x,
            y: pos.y + 1,
        })
        .filter(|c| c.y < map.len()), // south
        pos.x.checked_sub(1).map(|x| Coord { x, y: pos.y }), // west
        Some(Coord {
            x: pos.x + 1,
            y: pos.y,
        })
        .filter(|c| c.x < map[0].len()), // east
    ]
}

// don't incur cost at start
fn get_route_cost(map: &Vec<Vec<u32>>, route: &[Coord]) -> u32 {
    route.iter().skip(1).map(|c| map[c.y][c.x]).sum::<u32>()
}

// Horrendously inefficient BFS, needs replacing
fn get_route_bfs(map: &Vec<Vec<u32>>, source: Coord, target: Coord) -> Vec<Coord> {
    let mut to_search = VecDeque::new();
    to_search.push_back((vec![source], 0));

    while !to_search.is_empty() {
        let (search_path, search_path_current_cost) = to_search.pop_front().unwrap();
        let search_node = search_path.last().unwrap();

        if *search_node == target {
            //println!("{:?}", search_path);
            return search_path;
        }

        for adj in get_adjacents(map, *search_node).iter().flatten() {
            // no reversing
            if search_path.contains(adj) {
                continue;
            }
            if search_path.len() > 3 {
                let ix = search_path.len();
                let parent1 = search_path[ix - 1];
                let parent2 = search_path[ix - 2];
                let parent3 = search_path[ix - 3];
                if (adj.x == parent1.x && parent1.x == parent2.x && parent2.x == parent3.x)
                    || (adj.y == parent1.y && parent1.y == parent2.y && parent2.y == parent3.y)
                {
                    continue;
                }
            }
            let mut v = search_path.clone();
            v.push(*adj);
            let i =
                to_search.partition_point(|a| a.1 < search_path_current_cost + map[adj.y][adj.x]);
            to_search.insert(i, (v, search_path_current_cost + map[adj.y][adj.x]));
        }
    }
    panic!("Could not find path");
}

fn get_route(map: &Vec<Vec<u32>>, source: Coord, target: Coord) -> Vec<Coord> {
    let mut came_from: HashMap<(Coord, u32, u32), Coord> = HashMap::new();
    let mut open_set = VecDeque::new();
    open_set.push_back((source, 0, 0));

    let mut g_score = HashMap::new();
    g_score.insert(open_set[0], 0); // don't incur heat loss at start

    while !open_set.is_empty() {
        let current_triple = open_set.pop_front().unwrap();
        let (current, move_x_count, move_y_count) = current_triple;
        if move_x_count > 3 || move_y_count > 3 {
            continue;
        }
        //println!("current: {current:?} {move_x_count} {move_y_count}");
        if current == target {
            break;
        }

        for adj in get_adjacents(map, current).iter().flatten() {
            let new_x_move_count = if adj.x == current.x {
                move_x_count + 1
            } else {
                0
            };
            let new_y_move_count = if adj.y == current.y {
                move_y_count + 1
            } else {
                0
            };
            let next_triple = (*adj, new_x_move_count, new_y_move_count);
            let dist = g_score[&current_triple] + map[adj.y][adj.x];
            println!("  {adj:?} {dist}");
            if &dist < g_score.get(&next_triple).unwrap_or(&u32::max_value()) {
                came_from.insert(next_triple, current);
                g_score.insert(next_triple, dist);
                open_set.push_back((*adj, new_x_move_count, new_y_move_count));
            }
        }
    }
    let mut total_path = vec![target];
    let mut current = target;
    while came_from.contains_key(&current) {
        println!("{current:?}");
        current = came_from[&current];
        total_path.push(current);
    }
    total_path.reverse();
    return total_path;
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;

    let input_str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    let map: Vec<Vec<u32>> = input_str
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let route = get_route(
        &map,
        Coord { x: 0, y: 0 },
        Coord {
            x: map[0].len() - 1,
            y: map.len() - 1,
        },
    );

    println!("{:?}", route);
    println!("Total route cost: {}", get_route_cost(&map, &route));

    Ok(())
}
