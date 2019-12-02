use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let mut input = match File::open(&env::args().nth(1).unwrap()) {
        Err(why) => panic!("Could not open input file: {}\n", why),
        Ok(fh) => fh,
    };

    let mut fstr = String::new();
    input.read_to_string(&mut fstr).unwrap();
    let instructions: Vec<_> = fstr.split(',').map(|dir| dir.trim()).collect();

    let mut cur_dir = 0; // North
    let mut locs = vec![(0, 0)]; // Starting point
    let mut visited: Option<(i32, i32)> = None;
    for ins in &instructions {
        let dir = ins.chars().nth(0);
        cur_dir = match dir.unwrap() {
            'L' => (cur_dir + (4 - 1)) % 4,
            'R' => (cur_dir + 1) % 4,
            _ => panic!("Weird turning direction: {:?}\n", dir),
        };

        let dist: i32 = ins.chars().skip(1).collect::<String>().parse().unwrap();
        let old_pos = locs.last().unwrap().clone();
        for i in 1..dist + 1 {
            let cur_pos = match cur_dir {
                0 => (old_pos.0 + i, old_pos.1),
                1 => (old_pos.0, old_pos.1 + i),
                2 => (old_pos.0 - i, old_pos.1),
                3 => (old_pos.0, old_pos.1 - i),
                _ => panic!("Current direction is not a direction: {}", cur_dir),
            };

            // See if we've visited this point before, if we haven't already found a point
            if locs.iter().any(|&l| l == cur_pos) {
                visited = visited.or(Some(cur_pos));
            }
            locs.push(cur_pos);
        }
    }

    let l = locs.last().unwrap();
    println!("Final distance: {} blocks", l.0.abs() + l.1.abs());
    let v = visited.unwrap();
    println!("Visited twice: {} blocks", v.0.abs() + v.1.abs());
}
