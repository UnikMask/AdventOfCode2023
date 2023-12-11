use aoc::load_file;

fn main() {
    let contents: Vec<Vec<bool>> = load_file("resources/day11/actual.txt")
        .iter()
        .map(|row| row.chars().map(|c| c == '#').collect())
        .collect();

    // Get empty columns and rows
    let (m, n) = (contents.len(), contents[0].len());
    let empty_rows: Vec<usize> = (0..m)
        .filter_map(|i| match (0..n).find(|j| contents[i][*j]) {
            None => Some(i),
            Some(_) => None,
        })
        .collect();
    let empty_cols: Vec<usize> = (0..n)
        .filter_map(|j| match (0..m).find(|i| contents[*i][j]) {
            None => Some(j),
            Some(_) => None,
        })
        .collect();
    let (rown, coln) = (empty_rows.len(), empty_cols.len());

    // Get galaxies for any age
    let get_galaxies = |age: usize| -> Vec<(i64, i64)> {
        let (age, mut rowi, mut res) = (age - 1, 0, vec![]);
        for i in 0..m {
            if rowi < rown && empty_rows[rowi] == i {
                rowi += 1;
                continue;
            }
            let mut coli = 0;
            for j in 0..n {
                if coli < coln && empty_cols[coli] == j {
                    coli += 1;
                } else if contents[i][j] {
                    res.push(((i + rowi * age) as i64, (j + coli * age) as i64));
                }
            }
        }
        res
    };

    // Get sum of distance between all pairs of galaxies
    let get_dist = |galaxies: Vec<(i64, i64)>| -> i64 {
        let mut total = 0;
        for i in 0..galaxies.len() - 1 {
            for j in i + 1..galaxies.len() {
                let (g0, g1) = (galaxies[i], galaxies[j]);
                total += (g0.0 - g1.0).abs() + (g0.1 - g1.1).abs();
            }
        }
        total
    };

    // Get distance between all pairs
    println!("Part 1 result: {}", get_dist(get_galaxies(2)));
    println!("Part 2 result: {}", get_dist(get_galaxies(1000000)));
}
