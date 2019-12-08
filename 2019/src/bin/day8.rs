use std::char;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

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

    let image_layers: Vec<_> = image_data.chunks(25 * 6).collect();

    let min_zero_layer = image_layers
        .iter()
        .min_by_key(|l| l.iter().filter(|&&c| c == 0).count())
        .unwrap();

    let checksum = min_zero_layer.into_iter().filter(|&&c| c == 1).count()
        * min_zero_layer.into_iter().filter(|&&c| c == 2).count();
    println!("Image checksum: {}", checksum);

    let output_image = image_layers.iter().fold([2; 25 * 6], |mut img, l| {
        for i in 0..25 * 6 {
            if img[i] == 2 {
                img[i] = l[i];
            }
        }
        img
    });
    println!("Output image:");
    for line in output_image.chunks(25) {
        for c in line {
            print!(
                "{}",
                if *c == 1 {
                    char::from_digit(*c, 10).unwrap()
                } else {
                    ' '
                }
            );
        }
        println!("");
    }
    Ok(())
}
