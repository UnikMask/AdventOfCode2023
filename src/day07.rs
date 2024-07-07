use aoc::load_file;

fn rank(card: char, joker: bool) -> usize {
    match card {
        'T' => 8 + joker as usize,
        'J' => !joker as usize * 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => card.to_digit(10).unwrap() as usize - 1 - !joker as usize,
    }
}

fn get_hash(hand: &str, joker: bool) -> u32 {
    let (mut hash, mut counter): (u32, [u8; 13]) = (0, [0; 13]);
    hand.chars().enumerate().for_each(|(i, card)| {
        let crank = rank(card, joker);
        hash += (crank << ((4 - i) * 4)) as u32;
        counter[crank] += 1;
    });

    let highest = (1..counter.len())
        .max_by(|a, b| counter[*a].cmp(&counter[*b]))
        .unwrap();
    hash += ((joker as usize)..counter.len())
        .map(
            |i| match counter[i] + ((joker && highest == i) as u8) * counter[0] {
                5.. => 6,
                4 => 5,
                3 => 3,
                2 => 1,
                0..=1 => 0,
            } << 20
        )
        .sum::<u32>();
    hash
}

fn main() {
    let get_contents = |jokers: bool| -> Vec<(u32, u32)> {
        load_file("resources/day07/actual.txt")
            .iter()
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                (get_hash(parts[0], jokers), parts[1].parse::<u32>().unwrap())
            })
            .collect()
    };

    let get_res = |jokers: bool| -> usize {
        let mut contents = get_contents(jokers);
        contents.sort_by(|a, b| a.0.cmp(&b.0));
        contents
            .iter()
            .enumerate()
            .map(|(i, e)| (i + 1) * e.1 as usize)
            .sum()
    };

    println!("Part 1 result: {}", get_res(false));
    println!("Part 2 result: {}", get_res(true));
}
