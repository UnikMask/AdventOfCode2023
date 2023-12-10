use aoc::load_file;

fn main() {
    println!("Result: {:?}", load_file("resources/day09/actual.txt")
        .iter()
        .map(|line| {
            let mut transforms = vec![line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()];

            while !transforms.last().unwrap().iter().all(|i| *i == 0) {
                let cur = transforms.last().unwrap().clone();
                transforms.push((0..cur.len() - 1).map(|i| cur[i + 1] - cur[i]).collect());
            }
            transforms
                .iter()
                .rev()
                .map(|t| (*t.first().unwrap(), *t.last().unwrap()))
                .reduce(|res, t| (t.0 - res.0, t.1 + res.1))
                .unwrap()
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap());
}
