use std::cmp;
use std::env;
use std::fs;
use std::io;
use std::str;

struct Record(String, Vec<usize>);

//fn count_arrangements_slow(record: &str, expected_groups: &[usize]) -> usize {
//    if let Some(idx) = record.rfind('?') {
//        let mut r1: Vec<_> = record.as_bytes().iter().copied().collect();
//        r1[idx] = b'#';
//        let mut r2: Vec<_> = record.as_bytes().iter().copied().collect();
//        r2[idx] = b'.';
//        count_arrangements_slow(str::from_utf8(r1.as_slice()).unwrap(), expected_groups)
//            + count_arrangements_slow(str::from_utf8(r2.as_slice()).unwrap(), expected_groups)
//    } else {
//        if record
//            .split('.')
//            .map(|s| s.len())
//            .filter(|&n| n > 0)
//            .collect::<Vec<_>>()
//            == expected_groups
//        {
//            1
//        } else {
//            0
//        }
//    }
//}

fn count_arrangements(record: &str, expected_groups: &[usize]) -> usize {
    //println!("{:?} {:?}", record, expected_groups);
    if expected_groups.is_empty() {
        // we managed to fit in all groups, successful arrangement
        return if record.contains('#') { 0 } else { 1 };
    }

    if record.is_empty() {
        // ran out of record, unsuccessful arrangement
        return 0;
    }

    // find a place for the next group
    let next_group_len = expected_groups[0];
    if let Some(next_possible_position) = record.find(['#', '?']) {
        let group_plus_remainder = &record[next_possible_position..];
        if group_plus_remainder.len() < next_group_len {
            //println!("ret1");
            // not enough string left, unsuccessful arrangement
            return 0;
        }

        let possible_group = &group_plus_remainder[..next_group_len];

        let contains_corruption = possible_group.starts_with('?');

        if possible_group.contains('.') {
            // can't fit group in this position, unsuccessful arrangement
            //println!("ret2");
            return if contains_corruption {
                count_arrangements(&group_plus_remainder[1..], expected_groups)
            } else {
                0
            };
        }

        if group_plus_remainder[next_group_len..].starts_with('#') {
            // no gap after end of group, unsuccessful arrangement
            //println!("ret3");
            return if contains_corruption {
                count_arrangements(&group_plus_remainder[1..], expected_groups)
            } else {
                0
            };
        }

        // if there's stuff left after this group, skip a position to ensure a gap
        let after_group =
            &group_plus_remainder[cmp::min(next_group_len + 1, group_plus_remainder.len())..];

        // successfully placed a group, get combinations with this group removed
        let arrangements_after_group_placement =
            count_arrangements(after_group, &expected_groups[1..]);

        // if we say a group doesn't go here, try the next position with the same groups
        arrangements_after_group_placement
            + if contains_corruption {
                count_arrangements(&group_plus_remainder[1..], expected_groups)
            } else {
                0
            }
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

    //for Record(rec, groups) in &records {
    //    let new = count_arrangements(rec, groups);
    //    let old = count_arrangements_slow(rec, groups);
    //    if old != new {
    //        println!("{:?} {:?} -> {} / {}", rec, groups, old, new);
    //    }
    //}

    let arrangement_sum: usize = records.iter().map(|r| count_arrangements(&r.0, &r.1)).sum();

    println!("Sum of possible arrangements: {}", arrangement_sum);

    // Multiply by 5
    let uncoiled_records: Vec<_> = records
        .iter()
        .map(|r| {
            Record(
                r.0.repeat(5),
                r.1.iter().cycle().take(r.1.len() * 5).copied().collect(),
            )
        })
        .collect();

    let uncoiled_arrangement_sum: usize = uncoiled_records
        .iter()
        .map(|r| count_arrangements(&r.0, &r.1))
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
        for (record, expected_groups) in TEST_DATA1 {
            assert_eq!(count_arrangements(record, expected_groups), 1);
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
        assert_eq!(count_arrangements("??", &[1]), 2);
        assert_eq!(count_arrangements("????", &[1, 1]), 3);

        assert_eq!(count_arrangements("??.#", &[1, 1]), 2);
        assert_eq!(count_arrangements("??", &[1]), 2);
        assert_eq!(count_arrangements(".??", &[1]), 2);

        assert_eq!(count_arrangements("?#?", &[1]), 1);
    }

    #[test]
    fn ex2b() {
        for (record, expected_groups, expected_combinations) in TEST_DATA2 {
            assert_eq!(
                count_arrangements(record, expected_groups),
                *expected_combinations
            );
        }
    }

    #[test]
    fn ex3() {
        assert_eq!(count_arrangements("??#?.???##", &[1, 4]), 1);
    }
}
