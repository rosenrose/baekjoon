use std::io;

const MAX: usize = 50;
const DIRS: [(i32, i32); 8] = [
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let [n, m] = [(); 2].map(|_| input() as usize);
    let mut map = [[0; MAX]; MAX];

    for r in 0..n {
        for c in 0..n {
            map[r][c] = input();
        }
    }

    let moves = (0..m).map(|_| (input() as usize - 1, input()));
    let sum = simulate(&mut map[..n], moves);

    println!("{sum}");
}

fn simulate(map: &mut [[i32; MAX]], moves: impl Iterator<Item = (usize, i32)>) -> i32 {
    let n = map.len();
    let mut clouds = [(0, 0); MAX * MAX];
    clouds[..4].copy_from_slice(&[(n - 1, 0), (n - 1, 1), (n - 2, 0), (n - 2, 1)]);
    let mut clouds_len = 4;

    let mut is_cloud = [[false; MAX]; MAX];

    for (dir, speed) in moves {
        for (r, c) in &mut clouds[..clouds_len] {
            (*r, *c) = (
                (*r as i32 + DIRS[dir].0 * speed).rem_euclid(n as i32) as usize,
                (*c as i32 + DIRS[dir].1 * speed).rem_euclid(n as i32) as usize,
            );

            map[*r][*c] += 1;
            is_cloud[*r][*c] = true;
        }

        water_copy(map, &clouds[..clouds_len]);

        let mut new_clouds = [(0, 0); MAX * MAX];
        let mut new_clouds_len = 0;

        for r in 0..n {
            for c in 0..n {
                if map[r][c] < 2 || is_cloud[r][c] {
                    continue;
                }

                map[r][c] -= 2;
                new_clouds[new_clouds_len] = (r, c);
                new_clouds_len += 1;
            }
        }

        for &(r, c) in &clouds[..clouds_len] {
            is_cloud[r][c] = false;
        }

        clouds[..new_clouds_len].copy_from_slice(&new_clouds[..new_clouds_len]);
        clouds_len = new_clouds_len;
    }

    map.iter().flatten().sum()
}

fn water_copy(map: &mut [[i32; MAX]], clouds: &[(usize, usize)]) {
    let n = map.len() as i32;
    let mut waters = [((0, 0), 0); MAX * MAX];
    let mut waters_len = 0;

    for &(r, c) in clouds {
        let adjacents = [
            (r as i32 - 1, c as i32 - 1),
            (r as i32 - 1, c as i32 + 1),
            (r as i32 + 1, c as i32 - 1),
            (r as i32 + 1, c as i32 + 1),
        ];

        let water = adjacents
            .iter()
            .filter(|&&(adj_r, adj_c)| {
                (0 <= adj_r && adj_r < n && 0 <= adj_c && adj_c < n)
                    && map[adj_r as usize][adj_c as usize] > 0
            })
            .count() as i32;

        if water > 0 {
            waters[waters_len] = ((r, c), water);
            waters_len += 1;
        }
    }

    for &((r, c), water) in &waters[..waters_len] {
        map[r][c] += water;
    }
}
