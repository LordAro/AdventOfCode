use std::char;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const BLACK: u32 = 0;
const WHITE: u32 = 1;
const TRANSPARENT: u32 = 2;
const HEIGHT: usize = 6;
const WIDTH: usize = 25;

fn main() -> io::Result<()> {
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
    .map(|c| c.to_digit(10).unwrap())
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
        .fold([TRANSPARENT; WIDTH * HEIGHT], |mut img, l| {
            for i in 0..WIDTH * HEIGHT {
                if img[i] == TRANSPARENT {
                    img[i] = l[i];
                }
            }
            img
        });
    println!("Output image:");
    for line in output_image.chunks(WIDTH) {
        println!(
            "{}",
            line.iter()
                .map(|&c| if c == WHITE {
                    char::from_digit(c, 10).unwrap()
                } else {
                    ' '
                })
                .collect::<String>()
        );
    }
    Ok(())
}
