use aoc::load_file;

fn main() {
    // Part 1
    let contents = load_file("resources/day01/actual.txt");

    let bytes_to_int = |s: Vec<u8>| {
        let (mut l, mut r) = (0, s.len() - 1);
        while !(s[l] as char).is_digit(10) {
            l += 1
        }
        while !(s[r] as char).is_digit(10) {
            r -= 1
        }
        ((s[l] - 48) * 10 + (s[r] - 48)) as u32
    };

    let res: u32 = contents
        .iter()
        .map(|s| s.bytes().collect::<Vec<u8>>())
        .map(bytes_to_int)
        .sum();
    println!("Part 1 result: {}", res);

    // Part 2
    // Solution - replace words with digits
    // Better solutions out there (Sliding Window), but I didn't want to do 
    // that.
    let compares = vec![ ("one", "o1e"), ("two", "t2o"), ("three", "t3e"), ("four", "f4"), ("five", "f5e"), ("six", "s6"), ("seven", "s7n"), ("eight", "e8t"), ("nine", "n9e")];
    let res2: u32 = contents.iter().map(|s| {
        let mut sc = s.clone();
        for (pattern, rep) in compares.iter() {
            sc = sc.replace(pattern, rep);
        }
        sc.bytes().collect::<Vec<u8>>()
    }).map(bytes_to_int).sum();
    println!("Part 2 result: {}", res2)
}
