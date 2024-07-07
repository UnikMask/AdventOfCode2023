use aoc::load_file;

fn main() {
    // Get boat races
    let contents = load_file("resources/day06/actual.txt")
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|d| match d.parse::<f32>() {
                    Err(_) => None,
                    Ok(digit) => Some(digit),
                })
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>();
    let races = (0..contents[0].len()).map(|i| (contents[0][i], contents[1][i]));

    // Let:
    // - T: total allowed time
    // - D: target distance
    // - t: time taken pushing button
    // - d: Extra distance from target distance
    // d = (T - t) * t - D = - t^2 + T * t - D
    // (T - t) * t - D = 0 <=> t = (-T +- sqrt(T^2 - 4 * D)) / -2
    let res1: i32 = races
        .map(|(t, d)| {
            let minb = (-t + (t * t - 4.0 * d).sqrt()) / -2.0;
            let maxb = (-t - (t * t - 4.0 * d).sqrt()) / -2.0;
            (maxb.ceil() - minb.floor()) as i32 - 1
        })
        .product();
    println!("Part 1 result: {}", res1);

    let bigrace = load_file("resources/day06/actual.txt").iter().map(|line| {
        line.split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect::<String>()
            .parse::<f64>().unwrap()
    }).collect::<Vec<f64>>();
    let (t, d) = (bigrace[0], bigrace[1]);
    println!("{:?}", bigrace);
    let minb = (-t + (t * t - 4.0 * d).sqrt()) / -2.0;
    let maxb = (-t - (t * t - 4.0 * d).sqrt()) / -2.0;
    println!("Part 2 result: {}", (maxb.ceil() - minb.floor()) as i64 - 1);
}
