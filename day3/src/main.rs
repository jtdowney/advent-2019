use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

type Point = (i32, i32);
const ORIGIN: Point = (0, 0);

fn part1(wire1: &HashMap<Point, u32>, wire2: &HashMap<Point, u32>) {
    let wire1 = wire1.keys().cloned().collect::<HashSet<Point>>();
    let wire2 = wire2.keys().cloned().collect::<HashSet<Point>>();
    let intersections = wire1.intersection(&wire2);
    let answer = intersections
        .filter(|&p| *p != ORIGIN)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .expect("a minimum");
    println!("part 1 = {}", answer);
}

fn part2(wire1: &HashMap<Point, u32>, wire2: &HashMap<Point, u32>) {
    let keys1 = wire1.keys().cloned().collect::<HashSet<Point>>();
    let keys2 = wire2.keys().cloned().collect::<HashSet<Point>>();
    let intersections = keys1.intersection(&keys2);
    let answer = intersections
        .filter(|&p| *p != ORIGIN)
        .map(|position| wire1[position] + wire2[position])
        .min()
        .expect("a minimum");
    println!("part 2 = {}", answer);
}

fn main() {
    let wires = io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(|line| {
            let mut initial = HashMap::new();
            initial.insert(ORIGIN, 0);

            let (acc, _) = line.split(',').map(|path| path.to_string()).fold(
                (initial, ORIGIN),
                |(mut acc, previous), path| {
                    let mut chars = path.chars();
                    let direction = chars.next().expect("direction");
                    let length = chars.collect::<String>().parse().expect("length");
                    let (offset_x, offset_y) = match direction {
                        'R' => (1, 0),
                        'L' => (-1, 0),
                        'U' => (0, 1),
                        'D' => (0, -1),
                        _ => unreachable!(),
                    };

                    let steps = acc[&previous];
                    let mut position = previous;
                    for i in 1..=length {
                        let (position_x, position_y) = position;
                        position = (position_x + offset_x, position_y + offset_y);
                        acc.insert(position, steps + i);
                    }

                    (acc, position)
                },
            );
            acc
        })
        .collect::<Vec<HashMap<Point, u32>>>();

    part1(&wires[0], &wires[1]);
    part2(&wires[0], &wires[1]);
}
