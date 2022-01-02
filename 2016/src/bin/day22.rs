extern crate itertools;
extern crate regex;

use itertools::iproduct;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct StorageNode {
    total: usize,
    used: usize,
}

impl StorageNode {
    fn avail(&self) -> usize {
        self.total - self.used
    }
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

    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .filter_map(|li| {
            let l = li.unwrap();
            grid_re.captures(&l).map(|caps| {
                (
                    (
                        caps.at(1).unwrap().parse::<usize>().unwrap(),
                        caps.at(2).unwrap().parse::<usize>().unwrap(),
                    ),
                    StorageNode {
                        total: caps.at(3).unwrap().parse::<usize>().unwrap(),
                        used: caps.at(4).unwrap().parse::<usize>().unwrap(),
                    },
                )
            })
        })
        .collect::<Vec<_>>();

    let valid_nodes = iproduct!(input.iter(), input.iter())
        .filter(|((pos_a, node_a), (pos_b, node_b))| {
            pos_a != pos_b && node_a.used > 0 && node_b.avail() > node_a.used
        })
        .count();

    println!("Valid nodes: {}", valid_nodes);
}
