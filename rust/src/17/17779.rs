use std::io;

const MAX: usize = 20;
const DISTRICTS: usize = 5;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut total_population = 0;
    let mut map = [[0; MAX]; MAX];

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            total_population += num;
            map[r][c] = num;
        }
    }

    let mut min_diff = i32::MAX;

    for y in 0..n - 2 {
        for x in 1..n - 1 {
            for d1 in 1..(x + 1).min(n - (y + 1)) {
                for d2 in 1..(n - x).min(n.saturating_sub(y + d1)) {
                    min_diff =
                        get_diff((y, x), (d1, d2), &map[..n], total_population).min(min_diff);
                }
            }
        }
    }

    println!("{min_diff}");
}

fn get_diff(
    (y, x): (usize, usize),
    (d1, d2): (usize, usize),
    map: &[[i32; MAX]],
    total_population: i32,
) -> i32 {
    let n = map.len();
    let mut populations = [0; DISTRICTS + 1];
    let mut borders = [[false; MAX]; MAX];
    let mut border = (y, x);

    for _ in 0..d1 {
        borders[border.0][border.1] = true;
        border.0 += 1;
        border.1 -= 1;
    }
    for _ in 0..d2 {
        borders[border.0][border.1] = true;
        border.0 += 1;
        border.1 += 1;
    }
    for _ in 0..d1 {
        borders[border.0][border.1] = true;
        border.0 -= 1;
        border.1 += 1;
    }
    for _ in 0..d2 {
        borders[border.0][border.1] = true;
        border.0 -= 1;
        border.1 -= 1;
    }

    for r in 0..y + d1 {
        for c in 0..=x {
            if borders[r][c] {
                break;
            }
            populations[1] += map[r][c];
        }
    }
    for r in 0..=y + d2 {
        let mut sum = 0;

        for c in x + 1..n {
            if borders[r][c] {
                sum = 0;
                continue;
            }
            sum += map[r][c];
        }

        populations[2] += sum;
    }
    for r in y + d1..n {
        for c in 0..x - d1 + d2 {
            if borders[r][c] {
                break;
            }
            populations[3] += map[r][c];
        }
    }
    for r in y + d2 + 1..n {
        let mut sum = 0;

        for c in x - d1 + d2..n {
            if borders[r][c] {
                sum = 0;
                continue;
            }
            sum += map[r][c];
        }

        populations[4] += sum;
    }

    populations[5] = total_population - populations[1..=4].iter().sum::<i32>();
    populations.sort();

    populations[DISTRICTS] - populations[1]
}
