use std::time::Instant;

use aoc::load_file;

const MAX_SIZE: usize = 0xffff;
const START_Z: usize = 25 << 10;
const END_A: usize = 25 + (25 << 5);

fn hash(node: &str) -> u16 {
    node.chars()
        .filter_map(|c| match c {
            'A'..='Z' => Some(((c as u16) - ('A' as u16)) << 10),
            _ => None,
        })
        .reduce(|a, b| (a >> 5) + b)
        .unwrap()
}

fn gcd(a: &usize, b: &usize) -> usize {
    let (mut a, mut b) = (*a, *b);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn main() {
    let now = Instant::now();
    let mut nodes: [(u16, u16); MAX_SIZE] = [(0, 0); MAX_SIZE];
    let contents = load_file("resources/day08/actual.txt");
    let instructions: Vec<char> = contents[0].chars().collect();
    let n = instructions.len();

    (2..contents.len()).for_each(|i| {
        let parts = contents[i].split(" = ").collect::<Vec<&str>>();
        let dirs: Vec<u16> = parts[1].split(", ").map(hash).collect();
        nodes[hash(parts[0]) as usize] = (dirs[0], dirs[1]);
    });

    let next_item = |cur: usize, step: usize| match instructions[step % n] {
        'L' => nodes[cur].0,
        _ => nodes[cur].1,
    } as usize;
    let get_steps_to_z = |node: usize| {
        let (mut cur, mut steps) = (node, 0);
        while cur < START_Z {
            (cur, steps) = (next_item(cur, steps), steps + 1);
        }
        steps
    };

    // Part 1
    println!("Part 1 result: {}", get_steps_to_z(0));

    // Part 2 -- collect starting nodes, get steps for each,
    //           Get minimal nb. steps until they're aligned.
    //           #steps = a / GCD(a, b) * B
    let res2 = (0..END_A)
        .filter_map(|i| match nodes[i] {
            (0, 0) => None,
            _ => Some(get_steps_to_z(i)),
        })
        .reduce(|a, b| a / gcd(&a, &b) * b)
        .unwrap();
    println!("Part 2 result: {}", res2);
    println!("Time elapsed: {}us", now.elapsed().as_micros());
}
