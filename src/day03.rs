use std::collections::HashSet;

use aoc::load_file;

fn main() {
    let contents = load_file("resources/day03/actual.txt");

    // step 1 - collect numbers and parts
    let (mut numbers, mut parts): (Vec<u32>, Vec<(usize, usize)>) = (Vec::new(), Vec::new());
    let mut grid: Vec<Vec<Option<u32>>> = vec![Vec::with_capacity(contents[0].len()); contents.len()];
    let (mut gears, mut i): (Vec<(usize, usize)>, usize) = (Vec::new(), 0);
    for line in contents {
        let (mut state, mut j): (Option<u32>, usize) = (None, 0);
        for c in line.chars() {
            let mut item = None;
            state = match c {
                '.' => None,
                '0'..='9' => { // Digit
                    match state {
                        None => numbers.push(c.to_digit(10).unwrap()),
                        Some(d) => {
                            *numbers.last_mut().unwrap() = d * 10 + c.to_digit(10).unwrap()
                        }
                    };
                    item = Some(numbers.len() as u32 - 1);
                    Some(*numbers.last().unwrap())
                }
                _ => { // Parts and gears
                    parts.push((i, j));
                    if c == '*' {
                        gears.push((i, j));
                    }
                    None
                }
            };
            grid[i].push(item);
            j += 1;
        }
        i += 1;
    }

    // Step 2 - find numbers part
    let mut part_numbers: Vec<bool> = vec![false; numbers.len()];
    parts.iter().for_each(|(i, j)| {
        for ii in -1..=1 {
            for jj in -1..=1 {
                let (x, y) = (*i as isize + ii, *j as isize + jj);
                if x < 0 || x >= grid[0].len() as isize || y < 0 || y >= grid.len() as isize {
                    continue;
                } else if let Some(i) = grid[x as usize][y as usize] {
                    part_numbers[i as usize] = true;
                }
            }
        }
    });
    let res1: u32 = (0..numbers.len())
        .filter(|i| part_numbers[*i])
        .map(|i| numbers[i])
        .sum();
    println!("Part 1 res: {}", res1);

    let res2: u32 = gears.iter().filter_map(|(i, j)| {
        let (mut links, mut ratio) = (0, 1);
        let mut found: HashSet<u32> = HashSet::with_capacity(8);
        for ii in -1..=1 {
            for jj in -1..=1 {
                let (x, y) = (*i as isize + ii, *j as isize + jj);
                if x < 0 || x >= grid[0].len() as isize || y < 0 || y >= grid.len() as isize {
                    continue;
                } else if let Some(i) = grid[x as usize][y as usize] {
                    if found.contains(&i) {
                        continue;
                    }
                    links += 1;
                    ratio *= numbers[i as usize];
                    found.insert(i);
                }
            }
        }
        if links < 2 {
            return None;
        }
        Some(ratio)
    }).sum();
    println!("Part 2 res: {}", res2);
}
