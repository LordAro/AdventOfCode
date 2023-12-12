use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::str;

extern crate itertools;
use itertools::Itertools;

struct Record(String, Vec<usize>);

fn count_arrangements(
    cache: &mut HashMap<(String, Vec<usize>), usize>,
    record: &str,
    expected_groups: &[usize],
) -> usize {
    if expected_groups.is_empty() {
        // we managed to fit in all groups, successful arrangement
        return if record.contains('#') { 0 } else { 1 };
    }

    if record.is_empty() {
        // ran out of record, unsuccessful arrangement
        return 0;
    }

    let cache_key = (String::from(record), expected_groups.to_vec());
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    // find a place for the next group
    let next_group_len = expected_groups[0];
    if let Some(next_possible_position) = record.find(['#', '?']) {
        let group_plus_remainder = &record[next_possible_position..];
        if group_plus_remainder.len() < next_group_len {
            // not enough string left, unsuccessful arrangement
            return 0;
        }

        let possible_group = &group_plus_remainder[..next_group_len];

        let is_corrupted = possible_group.starts_with('?');

        if possible_group.contains('.') {
            // can't fit group in this position, unsuccessful arrangement
            let ret = if is_corrupted {
                count_arrangements(cache, &group_plus_remainder[1..], expected_groups)
            } else {
                0
            };
            cache.insert(cache_key, ret);
            return ret;
        }

        if group_plus_remainder[next_group_len..].starts_with('#') {
            // no gap after end of group, unsuccessful arrangement
            let ret = if is_corrupted {
                count_arrangements(cache, &group_plus_remainder[1..], expected_groups)
            } else {
                0
            };
            cache.insert(cache_key, ret);
            return ret;
        }

        // if there's stuff left after this group, skip a position to ensure a gap
        let after_group =
            &group_plus_remainder[cmp::min(next_group_len + 1, group_plus_remainder.len())..];

        // successfully placed a group, get combinations with this group removed
        let arrangements_after_group_placement =
            count_arrangements(cache, after_group, &expected_groups[1..]);

        // if we say a group doesn't go here, try the next position with the same groups
        let ret = arrangements_after_group_placement
            + if is_corrupted {
                count_arrangements(cache, &group_plus_remainder[1..], expected_groups)
            } else {
                0
            };
        cache.insert(cache_key, ret);
        ret
    } else {
        // No more possible positions, unsuccessful arrangement
        0
    }
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;
    let records: Vec<_> = input_str
        .lines()
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            Record(
                String::from(it.next().unwrap()),
                it.next()
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    let mut cache = HashMap::new();

    let arrangement_sum: usize = records
        .iter()
        .map(|r| count_arrangements(&mut cache, &r.0, &r.1))
        .sum();

    println!("Sum of possible arrangements: {}", arrangement_sum);

    // Multiply by 5
    let uncoiled_records: Vec<_> = records
        .iter()
        .map(|r| {
            Record(
                itertools::repeat_n(r.0.clone(), 5).join("?"),
                r.1.iter().cycle().take(r.1.len() * 5).copied().collect(),
            )
        })
        .collect();

    let uncoiled_arrangement_sum: usize = uncoiled_records
        .iter()
        .map(|r| count_arrangements(&mut cache, &r.0, &r.1))
        .sum();

    println!(
        "Sum of possible uncoiled arrangements: {}",
        uncoiled_arrangement_sum
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_DATA1: &[(&str, &[usize])] = &[
        ("#.#.###", &[1, 1, 3]),
        (".#...#....###.", &[1, 1, 3]),
        (".#.###.#.######", &[1, 3, 1, 6]),
        ("####.#...#...", &[4, 1, 1]),
        ("#....######..#####.", &[1, 6, 5]),
        (".###.##....#", &[3, 2, 1]),
    ];

    #[test]
    fn ex1() {
        let mut cache = HashMap::new();
        for (record, expected_groups) in TEST_DATA1 {
            assert_eq!(count_arrangements(&mut cache, record, expected_groups), 1);
        }
    }

    const TEST_DATA2: &[(&str, &[usize], usize)] = &[
        ("???.###", &[1, 1, 3], 1),
        (".??..??...?##.", &[1, 1, 3], 4),
        ("?#?#?#?#?#?#?#?", &[1, 3, 1, 6], 1),
        ("????.#...#...", &[4, 1, 1], 1),
        ("????.######..#####.", &[1, 6, 5], 4),
        ("?###????????", &[3, 2, 1], 10),
    ];

    // manual tests based on group 2
    #[test]
    fn ex2a() {
        let mut cache = HashMap::new();
        assert_eq!(count_arrangements(&mut cache, "??", &[1]), 2);
        assert_eq!(count_arrangements(&mut cache, "????", &[1, 1]), 3);

        assert_eq!(count_arrangements(&mut cache, "??.#", &[1, 1]), 2);
        assert_eq!(count_arrangements(&mut cache, "??", &[1]), 2);
        assert_eq!(count_arrangements(&mut cache, ".??", &[1]), 2);

        assert_eq!(count_arrangements(&mut cache, "?#?", &[1]), 1);
    }

    #[test]
    fn ex2b() {
        let mut cache = HashMap::new();
        for (record, expected_groups, expected_combinations) in TEST_DATA2 {
            assert_eq!(
                count_arrangements(&mut cache, record, expected_groups),
                *expected_combinations
            );
        }
    }

    #[test]
    fn ex3() {
        let mut cache = HashMap::new();
        assert_eq!(count_arrangements(&mut cache, "??#?.???##", &[1, 4]), 1);
    }
}
