use aoc::load_file;

fn main() {
    let contents = load_file("resources/day02/actual.txt");
    let setup: Vec<u32> = vec![12, 13, 14];
    let get_index = |color: &str| match color {
        "green" => 1,
        "blue" => 2,
        _ => 0,
    };

    // Parse content
    let (mut sum1, mut sum2) = (0, 0);
    for game in contents {
        let parts: Vec<&str> = game.split(": ").collect();
        let (mut minv, mut bad): (Vec<u32>, bool) = (vec![0, 0, 0], false);
        for round in parts[1].split("; ") {
            for drawn_ball in round.split(", ") {
                let drawn: Vec<&str> = drawn_ball.split_whitespace().collect();
                let (quantity, i) = (drawn[0].parse().unwrap(), get_index(drawn[1]));
                if quantity > setup[i] {
                    bad = true;
                }
                minv[i] = std::cmp::max(minv[i], quantity);
            }
        }
        if !bad {
            sum1 += parts[0]
                .split(" ")
                .nth(1)
                .unwrap()
                .to_string()
                .parse::<u32>()
                .unwrap();
        }
        sum2 += minv.iter().map(|i| *i).product::<u32>();
    }
    println!("Part 1: {}, Part 2: {}", sum1, sum2);
}
