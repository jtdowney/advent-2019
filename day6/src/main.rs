use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::iter;

fn part1(orbits: &HashMap<String, String>) {
    let answer: usize = orbits
        .keys()
        .map(|start| iter::successors(Some(start), |prev| orbits.get(prev.as_str())).count() - 1)
        .sum();

    println!("part 1 = {}", answer);
}

fn part2(orbits: &HashMap<String, String>) {
    let you_orbit = iter::successors(Some("YOU".to_string()), |prev| {
        orbits.get(prev.as_str()).cloned()
    })
    .collect::<Vec<String>>();
    let san_orbit = iter::successors(Some("SAN".to_string()), |prev| {
        orbits.get(prev.as_str()).cloned()
    })
    .collect::<Vec<String>>();

    let san_ancestors = san_orbit.iter().cloned().collect::<HashSet<String>>();
    let ancestor = you_orbit
        .iter()
        .find(|orbit| san_ancestors.contains(orbit.as_str()))
        .unwrap();

    let transwers_to_ancestor = you_orbit
        .iter()
        .take_while(|orbit| orbit != &ancestor)
        .count()
        - 1;
    let transwers_from_ancestor = san_orbit
        .iter()
        .take_while(|orbit| orbit != &ancestor)
        .count()
        - 1;
    let answer = transwers_to_ancestor + transwers_from_ancestor;

    println!("part 2 = {}", answer);
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read data"))
        .map(|line| {
            let mut parts = line.split(')');
            let value = parts.next().expect("value").to_string();
            let key = parts.next().expect("key").to_string();
            (key, value)
        })
        .collect::<HashMap<String, String>>();

    part1(&input);
    part2(&input);
}
