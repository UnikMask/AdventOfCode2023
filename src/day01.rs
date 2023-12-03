use aoc::load_file;

fn main() {
    // Part 1
    let contents = load_file("resources/day01/example2.txt");

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
    let compares = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let compares: Vec<(Vec<u8>, u32)> = compares
        .iter()
        .map(|(w, d)| (w.bytes().collect::<Vec<u8>>(), *d as u32))
        .collect();

    let res2: u32 = contents
        .iter()
        .map(|s| {
            let (mut first, mut last): (Option<u32>, Option<u32>) = (None, None);
            let mut window: Vec<u8> = Vec::with_capacity(5);

            // Sliding window through line
            for c in s.bytes() {
                if window.len() == 5 {
                    window = window[1..].to_vec();
                }
                window.push(c);

                // Find next digit if there is
                let mut found: Option<u32> = None;
                compares.iter().for_each(|(cmp, digit)| {
                    if window.len() >= cmp.len() && window[..cmp.len()] == *cmp.as_slice() {
                        found = Some(*digit);
                    }
                });
                if found == None && c.is_ascii_digit() {
                    found = Some(c as u32 - 48);
                }

                // Deal with found digit
                if let Some(digit) = found {
                    if first == None {
                        first = Some(digit);
                    }
                    last = Some(digit);
                }
            }
            first.unwrap() * 10 + last.unwrap()
        })
        .sum();
    println!("Part 2 result: {}", res2)
}
