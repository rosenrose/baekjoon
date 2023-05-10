use std::io;

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const SPREAD_RATIOS: [i32; 10] = [1, 1, 2, 2, 5, 7, 7, 10, 10, 0];
#[rustfmt::skip]
const SPREAD_COORDS: [[(i32, i32); 10]; 4] = [
    [(-1, 0), (1, 0), (-2, -1), (2, -1), (0, -3), (-1, -1), (1, -1), (-1, -2), (1, -2), (0, -2)],
    [(0, -1), (0, 1), (1, -2), (1, 2), (3, 0), (1, -1), (1, 1), (2, -1), (2, 1), (2, 0)],
    [(-1, 0), (1, 0), (-2, 1), (2, 1), (0, 3), (-1, 1), (1, 1), (-1, 2), (1, 2), (0, 2)],
    [(0, -1), (0, 1), (-1, -2), (-1, 2), (-3, 0), (-1, -1), (-1, 1), (-2, -1), (-2, 1), (-2, 0)],
];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let map: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();

    let sum = simulate(map);

    println!("{sum}");
}

fn simulate(mut map: Vec<Vec<i32>>) -> i32 {
    let n = map.len() as i32;
    let (mut r, mut c) = (n / 2, n / 2);
    let (mut dir, mut len) = (0, 1);
    let mut sum = 0;

    loop {
        for _ in 0..len {
            if (r, c) == (0, 0) {
                return sum;
            }

            let (next_r, next_c) = ((r + DIRS[dir].0) as usize, (c + DIRS[dir].1) as usize);
            let sand = map[next_r][next_c];

            for (&ratio, (mut adj_r, mut adj_c)) in SPREAD_RATIOS.iter().zip(SPREAD_COORDS[dir]) {
                adj_r += r;
                adj_c += c;

                let spread = if ratio == 0 {
                    map[next_r][next_c]
                } else {
                    sand * ratio / 100
                };

                map[next_r][next_c] -= spread;

                if adj_r < 0 || adj_r >= n || adj_c < 0 || adj_c >= n {
                    sum += spread;
                } else {
                    map[adj_r as usize][adj_c as usize] += spread;
                }
            }

            (r, c) = (next_r as i32, next_c as i32);
        }

        if dir & 1 == 1 {
            len += 1;
        }
        dir = (dir + 1) % DIRS.len();
    }
}
