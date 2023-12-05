use std::collections::HashSet;

use aoc::load_file;

fn main() {
    // File parsing
    let contents = load_file("resources/day04/actual.txt")
        .iter()
        .map(|line| {
            let sides: Vec<&str> = line.split(": ").last().unwrap().split(" | ").collect();
            let string_to_set = |strp: &str| {
                strp.split_whitespace()
                    .map(|i| i.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>()
            };
            string_to_set(sides[0])
                .intersection(&string_to_set(sides[1]))
                .count() as u32
        })
        .collect::<Vec<u32>>();

    // Part 1
    let res1: u32 = contents
        .iter()
        .map(|score| match score {
            0 => 0,
            _ => u32::pow(2, score - 1),
        })
        .sum();
    println!("Part 1 solution: {}", res1);

    // Part 2
    let (mut cards, n) = (vec![1 as u32; contents.len()], contents.len());
    let mut total = n as u32;
    for i in 0..n {
        let mut ii: usize = 1;
        total += cards[i] * contents[i];
        while ii <= contents[i] as usize {
            cards[i + ii] += cards[i];
            ii += 1;
        }
    }
    println!("Part 2 solution: {}", total);
}
