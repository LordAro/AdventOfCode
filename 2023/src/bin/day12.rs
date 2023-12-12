use std::env;
use std::fs;
use std::io;
use std::str;

#[derive(Debug)]
struct Record {
    record: String,
    group_lengths: Vec<usize>,
}

fn get_match(record: &str) -> Vec<usize> {
    record
        .split('.')
        .map(|s| s.len())
        .filter(|&n| n > 0)
        .collect()
}

fn count_possible_arrangements(record: &str, unmatched_groups: &[usize]) -> usize {
    if unmatched_groups.is_empty() {
        // base case - matched everything
        return 1;
    }

    let next_group_len = unmatched_groups[0];

    //if let Some(first_record_idx) = record.find('#') {
    //    //
    //    return count_possible_arrangements(
    //        &record[first_record_idx + next_group_len..],
    //        &unmatched_groups[1..],
    //    );
    //}

    return 0;
}

fn count_arrangements(record: &str, expected_groups: &[usize]) -> usize {
    if let Some(idx) = record.rfind('?') {
        let mut r1: Vec<_> = record.as_bytes().iter().copied().collect();
        r1[idx] = b'#';
        let mut r2: Vec<_> = record.as_bytes().iter().copied().collect();
        r2[idx] = b'.';
        count_arrangements(str::from_utf8(r1.as_slice()).unwrap(), expected_groups)
            + count_arrangements(str::from_utf8(r2.as_slice()).unwrap(), expected_groups)
    } else {
        if get_match(record) == expected_groups {
            1
        } else {
            0
        }
    }
}

fn main() -> io::Result<()> {
    let input_str = fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;
    let records: Vec<_> = input_str
        .lines()
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            Record {
                record: String::from(it.next().unwrap()),
                group_lengths: it
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect();

    //println!("{:?}", records);

    //count_possible_arrangements("#.#.###", &[1, 1, 3]);
    //println!("{:?}", get_match("#.#.###"));

    let arrangement_sum: usize = records
        .iter()
        .map(|r| count_arrangements(&r.record, &r.group_lengths))
        .sum();

    println!("Sum of possible arrangements: {}", arrangement_sum);

    let uncoiled_records: Vec<_> = records
        .iter()
        .map(|r| Record {
            record: r.record.repeat(5),
            group_lengths: r
                .group_lengths
                .iter()
                .cycle()
                .take(r.group_lengths.len() * 5)
                .copied()
                .collect(),
        })
        .collect();

    let uncoiled_arrangement_sum: usize = uncoiled_records
        .iter()
        .map(|r| count_arrangements(&r.record, &r.group_lengths))
        .sum();

    println!(
        "Sum of possible uncoiled arrangements: {}",
        uncoiled_arrangement_sum
    );

    Ok(())
}
