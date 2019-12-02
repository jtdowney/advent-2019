use std::io::{self, BufRead};
use std::str;

const PART2_RESULT: usize = 19_690_720;

fn execute_intcode(noun: usize, verb: usize, memory: &[usize]) -> usize {
    let mut memory = memory.to_vec();

    memory[1] = noun;
    memory[2] = verb;

    let mut ip = 0;
    loop {
        match memory[ip] {
            1 => {
                let value = memory[memory[ip + 1]] + memory[memory[ip + 2]];
                let address = memory[ip + 3];
                memory[address] = value;
                ip += 4;
            }
            2 => {
                let value = memory[memory[ip + 1]] * memory[memory[ip + 2]];
                let address = memory[ip + 3];
                memory[address] = value;
                ip += 4;
            }
            99 => break,
            _ => unreachable!(),
        }
    }

    memory[0]
}

fn part1(memory: &[usize]) {
    let answer = execute_intcode(12, 2, memory);
    println!("part 1 = {}", answer);
}

fn part2(memory: &[usize]) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = execute_intcode(noun, verb, memory);
            if result == PART2_RESULT {
                let answer = 100 * noun + verb;
                println!("part 2 = {}", answer);
                return;
            }
        }
    }
}

fn main() {
    let memory = io::stdin()
        .lock()
        .split(b',')
        .map(|item| item.expect("Failed to split input"))
        .map(|item| {
            str::from_utf8(&item)
                .expect("Failed to parse string")
                .trim()
                .parse()
                .expect("Failed to parse number")
        })
        .collect::<Vec<usize>>();

    part1(&memory);
    part2(&memory);
}
