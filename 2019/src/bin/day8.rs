use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const BLACK: char = '0';
const WHITE: char = '1';
const TRANSPARENT: char = '2';
const HEIGHT: usize = 6;
const WIDTH: usize = 25;

fn main() {
    let image_data: Vec<_> = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .chars()
    .collect();

    let image_layers: Vec<_> = image_data.chunks(WIDTH * HEIGHT).collect();

    let min_zero_layer = image_layers
        .iter()
        .min_by_key(|l| l.iter().filter(|&&c| c == BLACK).count())
        .unwrap();

    let checksum = min_zero_layer.into_iter().filter(|&&c| c == WHITE).count()
        * min_zero_layer
            .into_iter()
            .filter(|&&c| c == TRANSPARENT)
            .count();
    println!("Image checksum: {}", checksum);

    let output_image = image_layers
        .iter()
        .fold(vec![TRANSPARENT; WIDTH * HEIGHT], |img, l| {
            img.iter()
                .zip(l.iter())
                .map(|(&a, &b)| if a == TRANSPARENT { b } else { a })
                .collect()
        });
    println!("Output image:");
    for line in output_image.chunks(WIDTH) {
        println!(
            "{}",
            line.iter()
                .map(|&c| if c == WHITE { c } else { ' ' })
                .collect::<String>()
        );
    }
}
