use aoc::load_file;

fn main() {
    let contents = load_file("resources/day05/actual.txt");
    let seeds: Vec<u32> = contents[0]
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|i| i.parse::<u32>().unwrap())
        .collect();

    let maps: Vec<Vec<(u32, u32, u32)>> = contents[2..]
        .split(|line| line.is_empty())
        .map(|lines| {
            let mut map = lines[1..]
                .iter()
                .map(|line| {
                    let lvec = line
                        .split_whitespace()
                        .map(|i| i.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    (lvec[1], lvec[0], lvec[2])
                })
                .collect::<Vec<(u32, u32, u32)>>();
            map.sort();
            map
        }
        )
        .collect();

    let res1 = seeds.iter().map(|seed| {
        let mut res  = *seed;
        for map in &maps {
            let n = map.len();
            let (mut l, mut r): (i32, i32) = (0, n as i32 - 1);
            while l < r {
                let e = (l + r) / 2;
                let spot = &map[e as usize];
                if res > spot.0 + spot.2 - 1 {
                    l = e as i32 + 1;
                } else if res < spot.0 {
                    r = e as i32 - 1;
                } else {
                    r = e;
                    break;
                }
            }
            if r < 0 {
                continue;
            }
            let spot = &map[r as usize];
            if res > spot.0 && res - spot.0 < spot.2 {
                let nres = spot.1 + (res - spot.0);
                res = nres;
            }
        }
        (seed, res)
    }).min_by(|(_, resa), (_, resb)| resa.cmp(resb)).unwrap();
    println!("Part 1 result: {:?}", res1.1);
}
