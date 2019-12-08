use std::io::{self, Read};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn part1(layers: &[Vec<char>]) {
    let (layer, _) = layers
        .iter()
        .map(|layer| layer.iter().filter(|&c| *c == '0').count())
        .enumerate()
        .min_by_key(|&(_, count)| count)
        .unwrap();

    let ones = layers[layer].iter().filter(|&c| *c == '1').count();
    let twos = layers[layer].iter().filter(|&c| *c == '2').count();

    let answer = ones * twos;
    println!("part 1 = {}", answer);
}

fn part2(layers: &[Vec<char>]) {
    println!("part 2:");

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let offset = y * WIDTH + x;
            let pixel = layers
                .iter()
                .map(|layer| layer[offset])
                .find(|&pixel| pixel != '2');
            match pixel {
                Some('0') | None => print!(" "),
                Some('1') => print!("█"),
                _ => unreachable!(),
            }
        }

        println!();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("input");

    let chars = input.trim().chars().collect::<Vec<char>>();
    let layers = chars
        .chunks(WIDTH * HEIGHT)
        .map(|layer| layer.to_vec())
        .collect::<Vec<Vec<char>>>();

    part1(&layers);
    part2(&layers);
}
