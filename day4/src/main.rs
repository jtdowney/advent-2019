const MIN: u32 = 108_457;
const MAX: u32 = 562_041;

fn is_sorted(code: &str) -> bool {
    let mut sorted = code.as_bytes().to_vec();
    sorted.sort();
    sorted == code.as_bytes()
}

fn part1() {
    let answer = (MIN..=MAX)
        .map(|code| code.to_string())
        .filter(|code| code.as_bytes().windows(2).any(|part| part[0] == part[1]))
        .filter(|code| is_sorted(&code))
        .count();
    println!("part 1 = {}", answer);
}

fn part2() {
    let answer = (MIN..=MAX)
        .map(|code| code.to_string())
        .filter(|code| {
            let bytes = code.as_bytes();
            for i in 0..5 {
                let a = bytes[i];
                let b = bytes[i + 1];
                if a == b {
                    let neighbor_match = [-1i8, 2]
                        .iter()
                        .map(|&n| i as i8 + n)
                        .filter(|&n| n >= 0 && n < 6)
                        .map(|n| bytes[n as usize])
                        .any(|c| c == a);
                    if neighbor_match {
                        continue;
                    } else {
                        return true;
                    }
                }
            }

            false
        })
        .filter(|code| is_sorted(&code))
        .count();
    println!("part 2 = {}", answer);
}

fn main() {
    part1();
    part2();
}
