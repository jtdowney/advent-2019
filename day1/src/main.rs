use std::io::{self, BufRead};
use std::iter;

fn calculate_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn part1(input: &[i32]) {
    let answer: i32 = input.iter().map(|&mass| calculate_fuel(mass)).sum();
    println!("part 1 = {}", answer);
}

fn part2(input: &[i32]) {
    let answer: i32 = input
        .iter()
        .map(|&mass| {
            let fuel = calculate_fuel(mass);
            iter::successors(Some(fuel), |&prev_mass| {
                let fuel = calculate_fuel(prev_mass);
                if fuel > 0 {
                    Some(fuel)
                } else {
                    None
                }
            })
            .sum::<i32>()
        })
        .sum();

    println!("part 2 = {}", answer);
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read data"))
        .map(|line| line.parse().expect("Failed to parse number"))
        .collect::<Vec<i32>>();

    part1(&input);
    part2(&input);
}
