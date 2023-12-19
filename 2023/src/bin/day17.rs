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

    let mut g_score = HashMap::new();
    g_score.insert(source, map[source.y][source.x]);

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

        for adj in get_adjacents(map, current).iter().flatten() {
            if came_from.contains_key(&current) && *adj == came_from[&current] {
                // no reversing
                continue;
            }
            if came_from.contains_key(&current)
                && came_from.contains_key(&came_from[&current])
                && came_from.contains_key(&came_from[&came_from[&current]])
                && came_from.contains_key(&came_from[&came_from[&came_from[&current]]])
            {
                let parent1 = came_from[&current];
                let parent2 = came_from[&parent1];
                let parent3 = came_from[&parent2];
                let parent4 = came_from[&parent3];
                if (adj.x == current.x
                    && current.x == parent1.x
                    && parent1.x == parent2.x
                    && parent2.x == parent3.x
                    && parent2.x == parent4.x)
                    || (adj.y == current.y
                        && current.y == parent1.y
                        && parent1.y == parent2.y
                        && parent2.y == parent3.y
                        && parent3.y == parent4.y)
                {
                    // can't travel more than 3 blocks in the same direction
                    // but we count up to 4 to account for entry into a block
                    continue;
                }
            }
            let dist = g_score[&current] + map[adj.y][adj.x];
            if &dist < g_score.get(adj).unwrap_or(&u32::max_value()) {
                came_from.insert(*adj, current);
                g_score.insert(*adj, dist);
                open_set.push_back(*adj);
            }
        }
    }
    //print_positions(known_positions);
    panic!("Unable to find route between {:?} and {:?}", source, target);
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
