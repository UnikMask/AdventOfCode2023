use std::collections::VecDeque;

use aoc::load_file;

type Entry = (u8, isize, isize, usize, isize, isize);

// State machine for inside-outside test
enum InOut {
    S0, // Outside enclosed area
    S1(u32), // In enclosed area
    S2(char), // On loop horizontal line
}

fn is_continuous(c: char, i: isize, j: isize) -> bool {
    match (i, j) {
        (-1, 0) => vec!['|', 'F', '7'],
        (1, 0) => vec!['|', 'L', 'J'],
        (0, -1) => vec!['-', 'L', 'F'],
        (0, 1) => vec!['-', 'J', '7'],
        _ => vec![],
    }
    .contains(&c)
}

fn get_nbs(i: isize, j: isize, grid: &Vec<Vec<char>>) -> Vec<(isize, isize)> {
    match grid[i as usize][j as usize] {
        'S' => vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)],
        '|' => vec![(i - 1, j), (i + 1, j)],
        '-' => vec![(i, j - 1), (i, j + 1)],
        'L' => vec![(i - 1, j), (i, j + 1)],
        '7' => vec![(i, j - 1), (i + 1, j)],
        'F' => vec![(i, j + 1), (i + 1, j)],
        'J' => vec![(i, j - 1), (i - 1, j)],
        _ => vec![],
    }
}

fn neighbours(i: isize, j: isize) -> Vec<(isize, isize)> {
    return vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
}

// Transition function for inside outside test state machine
fn transition(state: InOut, input: Option<char>, total: &mut u32) -> InOut {
    match state {
        InOut::S0 => match input {
            None => InOut::S0,
            Some('|') => InOut::S1(0),
            Some(cur) => InOut::S2(cur),
        },
        InOut::S1(buf) => {
            *total += buf * (input != None) as u32;
            match input {
                None => InOut::S1(buf + 1),
                Some('L') => InOut::S2('F'),
                Some('F') => InOut::S2('L'),
                _ => InOut::S0,
            }
        }
        InOut::S2(c) => match input {
            Some('-') => InOut::S2(c),
            Some(cur) => match (c, cur) {
                ('F', '7') | ('L', 'J') => InOut::S0,
                _ => InOut::S1(0),
            },
            _ => InOut::S0,
        },
    }
}

fn in_bounds(i: isize, j: isize, m: usize, n: usize) -> bool {
    i >= 0 && j >= 0 && (i as usize) < m && (j as usize) < n
}

fn main() {
    let mut grid: Vec<Vec<char>> = load_file("resources/day10/actual.txt")
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    let (m, n) = (grid.len(), grid[0].len());

    let s: (isize, isize) = grid.iter().enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, c)| match c {
                'S' => Some((i as isize, j as isize)),
                _ => None,
            })
        }).unwrap();

    let mut steps: Vec<Vec<Option<(u8, usize)>>> = vec![vec![None; n]; m];
    let mut q: VecDeque<Entry> = VecDeque::from_iter(
        neighbours(s.0, s.1).iter().enumerate()
            .map(|(b, (i, j))| (b as u8, *i, *j, 1, s.0, s.1)),
    );

    // Part 1
    let (mut res1, mut branches) = (0, vec![]);
    while !q.is_empty() {
        let (branch, i, j, nsteps, ii, jj) = q.pop_front().unwrap();
        if !in_bounds(i, j, m, n) || !is_continuous(grid[i as usize][j as usize], i - ii, j - jj) {
            continue;
        }
        if let Some((obranch, lsteps)) = steps[i as usize][j as usize] {
            res1 = lsteps;
            branches = vec![branch, obranch];
            break;
        }
        steps[i as usize][j as usize] = Some((branch, nsteps));

        for (ii, jj) in get_nbs(i, j, &grid).iter().filter(|nb| (ii, jj) != **nb) {
            q.push_back((branch, *ii, *jj, nsteps + 1, i, j));
        }
    }
    println!("Part 1 result: {}", res1);

    // Replace S
    steps[s.0 as usize][s.1 as usize] = Some((branches[0], 0));
    let start_nbs = neighbours(s.0, s.1).iter()
        .map(|(i, j)| match in_bounds(*i, *j, m, n) {
            true => match steps[*i as usize][*j as usize] {
                None => false,
                Some((b, _)) => branches.contains(&b),
            },
            false => false,
        } as usize).reduce(|a, b| a << 1 + b).unwrap();
    grid[s.0 as usize][s.1 as usize] = match start_nbs {
        12 => '|', 10 => 'J', 9 => 'L', 6 => '7', 5 => 'F', _ => '-',
    };

    // Part 2 - find enclosed area via inside-outside test
    let mut res2: u32 = 0;
    for (i, row) in steps.iter().enumerate() {
        let mut state = InOut::S0;
        for (j, e) in row.iter().enumerate() {
            let input = match e {
                None => None,
                Some((b, _)) => match branches.contains(&b) {
                    true => Some(grid[i][j]),
                    false => None,
                } ,
            };
            state = transition(state, input, &mut res2);
        }
    }
    println!("Part 2 result: {}", res2);
}
