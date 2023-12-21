use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a.rem_euclid(b))
    }
}

fn main() {
    let asteroids: HashSet<_> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map_while(Result::ok)
    .enumerate()
    .flat_map(|(i, v)| {
        v.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(move |(j, _)| (j as isize, i as isize))
            .collect::<Vec<_>>()
    })
    .collect();

    let best_loc = asteroids
        .iter()
        .map(|s| {
            let source = *s;
            let seen: HashSet<_> = asteroids
                .iter()
                .filter(|&&t| source != t)
                .map(|t| {
                    let target = *t;
                    let normalised = (target.0 - source.0, target.1 - source.1);
                    let gcd = gcd(normalised.0, normalised.1);
                    let mut line = source;
                    loop {
                        line = (
                            (line.0 + (normalised.0 / gcd)),
                            (line.1 + (normalised.1 / gcd)),
                        );
                        if asteroids.contains(&line) {
                            break;
                        }
                    }
                    line
                })
                .collect();
            (source, seen.len())
        })
        .max_by_key(|(_, count)| *count)
        .unwrap();
    println!(
        "Best place for monitoring station is at {:?} which can see {} asteroids",
        best_loc.0, best_loc.1
    );

    let source = best_loc.0;
    let mut asteroid_angles: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    asteroids.iter().filter(|&&t| source != t).for_each(|t| {
        let target = *t;
        let normalised = (target.0 - source.0, target.1 - source.1);
        let gcd = gcd(normalised.0, normalised.1);
        let angle = (normalised.0 / gcd, normalised.1 / gcd);
        asteroid_angles.entry(angle).or_default().push(target);
    });

    asteroid_angles.iter_mut().for_each(|(_, val)| {
        val.sort_by(|a, b| {
            let hypot_a = ((a.0 - source.0) as f64).hypot((a.1 - source.1) as f64);
            let hypot_b = ((b.0 - source.0) as f64).hypot((b.1 - source.1) as f64);
            hypot_b.partial_cmp(&hypot_a).unwrap() // Order in reverse so we can pop_back
        })
    });

    let mut angle_keys: Vec<_> = asteroid_angles.keys().cloned().collect();
    angle_keys.sort_by(|a, b| {
        const PI: f64 = std::f64::consts::PI;
        let atan_a = ((a.1 as f64).atan2(a.0 as f64) - (1.5 * PI)).rem_euclid(PI * 2.0);
        let atan_b = ((b.1 as f64).atan2(b.0 as f64) - (1.5 * PI)).rem_euclid(PI * 2.0);
        atan_a.partial_cmp(&atan_b).unwrap()
    });

    let mut count = 0;
    for angle in angle_keys.iter().cycle() {
        let f = asteroid_angles.get_mut(angle).unwrap();
        if !f.is_empty() {
            let asteroid = f.pop();
            count += 1;
            if count == 200 {
                let asteroid = asteroid.unwrap();
                println!(
                    "Bet winning asteroid at {:?} ({})",
                    asteroid,
                    asteroid.0 * 100 + asteroid.1
                );
                break;
            }
        }
    }
}
