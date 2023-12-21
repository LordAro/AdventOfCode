extern crate itertools;
extern crate regex;

use itertools::iproduct;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct StorageNode {
    total: usize,
    used: usize,
    orig_pos: Coord,
}

impl StorageNode {
    fn avail(&self) -> usize {
        self.total - self.used
    }
}

type Filesystem = HashMap<Coord, StorageNode>;
type FSHashObj = (Coord, Coord); // current pos, orig_pos

fn filesystem_hashobj(fs: &Filesystem) -> Vec<FSHashObj> {
    fs.iter().map(|(pos, node)| (*pos, node.orig_pos)).collect()
}

fn fs_str(fs: &Filesystem) -> String {
    fs.iter().fold(String::new(), |mut output, (pos, node)| {
        let _ = write!(output, "({:?},{:?})", pos, node.orig_pos);
        output
    })
}

fn is_viable_copy(a: &StorageNode, b: &StorageNode) -> bool {
    a.used > 0 && b.avail() > a.used
}

fn adj(pos: &Coord) -> [Coord; 4] {
    [
        // nasty, but guaranteed to not be in the resulting map
        (pos.0, pos.1.overflowing_sub(1).0),
        (pos.0, pos.1 + 1),
        (pos.0.overflowing_sub(1).0, pos.1),
        (pos.0 + 1, pos.1),
    ]
}

fn move_data(
    cache: &mut HashSet<Vec<FSHashObj>>,
    fs: &Filesystem,
    target_data_pos: Coord,
    move_count: usize,
) -> usize {
    if fs.get(&(0, 0)).unwrap().orig_pos == target_data_pos {
        return move_count;
    }

    let fshash = filesystem_hashobj(fs);
    if cache.contains(&fshash) {
        println!("Seen {:?} before, skipping", fs_str(fs));
        return 1 << 32;
    }
    cache.insert(fshash);

    let possible_moves = fs
        .iter()
        .flat_map(|posnode_a| {
            adj(posnode_a.0)
                .iter()
                .filter(|&pos_b| fs.contains_key(pos_b))
                .map(|&pos_b| (*posnode_a.0, pos_b))
                .collect::<Vec<_>>()
        })
        .filter(|(pos_a, pos_b)| is_viable_copy(fs.get(pos_a).unwrap(), fs.get(pos_b).unwrap()))
        .collect::<Vec<_>>();
    //println!("{:?}", possible_moves);

    possible_moves
        .iter()
        .map(|mv| {
            let mut new_fs = fs.clone();
            let n0 = *new_fs.get(&mv.0).unwrap();
            let n1 = *new_fs.get(&mv.1).unwrap();
            new_fs.insert(mv.0, n1);
            new_fs.insert(mv.1, n0);
            println!("{:?} -> {:?}", fs_str(fs), fs_str(&new_fs));
            (new_fs, mv)
        })
        .map(|(new_fs, _)| move_data(cache, &new_fs, target_data_pos, move_count + 1))
        .min()
        .unwrap()
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    // id                     total   use  avail  use%
    // /dev/grid/node-x0-y0     85T   64T    21T   75%
    // groups:
    // 1 - x coord
    // 2 - y coord
    // 3 - total
    // 4 - use
    // 5 - avail
    // 6 - use%
    let grid_re =
        Regex::new(r".*-x([0-9]+)-y([0-9]+)\s+([0-9]+)T\s+([0-9]+)T\s+([0-9]+)T\s+([0-9]+)%")
            .unwrap();

    let initial_filesystem = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .filter_map(|li| {
            let l = li.unwrap();
            grid_re.captures(&l).map(|caps| {
                let pos = (
                    caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                );
                (
                    pos,
                    StorageNode {
                        total: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                        used: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                        orig_pos: pos,
                    },
                )
            })
        })
        .collect::<Filesystem>();

    let valid_nodes = iproduct!(initial_filesystem.iter(), initial_filesystem.iter())
        .filter(|((pos_a, node_a), (pos_b, node_b))| {
            pos_a != pos_b && is_viable_copy(node_a, node_b)
        })
        .count();

    println!("Valid nodes: {}", valid_nodes);

    let target_data_position = *initial_filesystem
        .keys()
        .filter(|(_, y)| *y == 0)
        .max_by_key(|(x, _)| x)
        .unwrap();

    let mut fs_state_cache = HashSet::new();
    let minimum_number_of_moves = move_data(
        &mut fs_state_cache,
        &initial_filesystem,
        target_data_position,
        1,
    );
    println!(
        "Minimum number of moves to get access to data: {}",
        minimum_number_of_moves
    );
}
