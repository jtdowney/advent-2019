use approx::relative_eq;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::f64::consts::{FRAC_PI_2, PI};
use std::io::{self, BufRead};

type Point = (u16, u16);

fn distance(a: Point, b: Point) -> f64 {
    ((f64::from(a.0) - f64::from(b.0)).powi(2) + (f64::from(a.1) - f64::from(b.1)).powi(2)).sqrt()
}

fn is_blocked(segment: &[Point], points: &HashSet<Point>) -> bool {
    let a = *segment.first().unwrap();
    let b = *segment.last().unwrap();
    let ab = distance(a, b);

    points.iter().filter(|&c| *c != a && *c != b).any(|&c| {
        let ac = distance(a, c);
        let bc = distance(b, c);
        relative_eq!(ac + bc, ab)
    })
}

fn find_station(asteroids: &HashSet<Point>) -> (Point, usize) {
    let counts: HashMap<Point, usize> = asteroids
        .iter()
        .cloned()
        .combinations(2)
        .filter(|segment| !is_blocked(segment, &asteroids))
        .flat_map(|segment| segment)
        .fold(HashMap::new(), |mut acc, point| {
            *acc.entry(point).or_insert(0) += 1;
            acc
        });

    counts.into_iter().max_by_key(|&(_, count)| count).unwrap()
}

fn part1(input: &HashSet<Point>) {
    let (_, answer) = find_station(&input);
    println!("part 1 = {}", answer);
}

fn part2(input: &HashSet<Point>) {
    let (station, _) = find_station(&input);
    let targets = input
        .iter()
        .filter(|&p| *p != station)
        .map(|&p| {
            let x = f64::from(p.0) - f64::from(station.0);
            let y = f64::from(p.1) - f64::from(station.1);
            let mut theta = y.atan2(x) + FRAC_PI_2;
            if theta < 0.0 {
                theta += 2.0 * PI;
            }

            (p, theta)
        })
        .sorted_by(|&(a, _), &(b, _)| {
            distance(station, a)
                .partial_cmp(&distance(station, b))
                .unwrap()
        })
        .sorted_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap())
        .group_by(|&(_, theta)| theta)
        .into_iter()
        .flat_map(|(_, group)| {
            group
                .enumerate()
                .map(|(i, (p, theta))| (p, (f64::from(i as u16) * PI * 2.0) + theta))
                .collect::<Vec<(Point, f64)>>()
        })
        .sorted_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap())
        .collect::<Vec<(Point, f64)>>();

    let ((x, y), _) = targets[199];
    let answer = x * 100 + y;
    println!("part 2 = {}", answer);
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("unable to read line"))
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .map(move |(x, _)| (x as u16, y as u16))
                .collect::<Vec<Point>>()
        })
        .collect::<HashSet<Point>>();

    part1(&input);
    part2(&input);
}
