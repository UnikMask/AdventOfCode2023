use aoc::load_file;

const N: usize = 0xffff;
const STARTZ: usize = 25 << 10;
const ENDA: usize = 25 + (25 << 5);

fn hash(node: &str) -> u16 {
    let mut hash: u16 = 0;
    node.chars().for_each(|c| match c {
        'A'..='Z' => {
            hash = (hash >> 5) + (((c as u16) - ('A' as u16)) << 10);
        }
        _ => (),
    });
    hash
}

fn gcd(a: &usize, b: &usize) -> usize {
    let (mut a, mut b) = (*a, *b);
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn main() {
    let mut nodes: [(u16, u16); N] = [(0, 0); N];
    let contents = load_file("resources/day08/actual.txt");
    let instructions: Vec<char> = contents[0].chars().collect();
    let n = instructions.len();
    contents[2..].iter().for_each(|node| {
        let parts = node.split(" = ").collect::<Vec<&str>>();
        let dirs: Vec<u16> = parts[1].split(", ").map(|node| hash(node)).collect();
        nodes[hash(parts[0]) as usize] = (dirs[0], dirs[1]);
    });

    let next_item = |cur: usize, step: usize| -> usize {
        return match instructions[step % n] {
            'L' => nodes[cur].0,
            _ => nodes[cur].1,
        } as usize;
    };

    // Part 1
    let (mut cur, mut steps) = (0, 0);
    while cur < STARTZ {
        cur = next_item(cur, steps);
        steps += 1
    }
    println!("Part 1 result: {}", steps);

    // Part 2 -- Assume each starting node has it's own ending node.
    let starts: Vec<usize> = (0..ENDA)
        .filter_map(|i| match nodes[i] {
            (0, 0) => None,
            _ => Some(i),
        })
        .collect();
    let res2 = starts
        .iter()
        .map(|start| {
            let (mut steps, mut cur) = (0, *start);
            while cur < STARTZ {
                cur = next_item(cur, steps);
                steps += 1;
            }
            steps
        })
        .reduce(|mut a, mut b| {
            if a < b {
                let tmp = b;
                b = a;
                a = tmp;
            }
            let c = gcd(&a, &b);
            return (a / c) * b;
        }).unwrap();
    println!("Part 2 result: {}", res2);
}
