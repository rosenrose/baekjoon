use std::io;

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

    let (n, m) = (input() as usize, input());
    let map: Vec<Vec<_>> = (0..n).map(|_| (0..n).map(|_| input()).collect()).collect();
    let moves = (0..m).map(|_| (input() as usize - 1, input()));

    let sum = simulate(map, moves);

    println!("{sum}");
}

fn simulate(mut map: Vec<Vec<i32>>, moves: impl Iterator<Item = (usize, i32)>) -> i32 {
    let n = map.len();
    let mut clouds = vec![(n - 1, 0), (n - 1, 1), (n - 2, 0), (n - 2, 1)];
    let mut is_cloud = vec![vec![false; n]; n];

    for (dir, speed) in moves {
        for (r, c) in &mut clouds {
            (*r, *c) = (
                (*r as i32 + DIRS[dir].0 * speed).rem_euclid(n as i32) as usize,
                (*c as i32 + DIRS[dir].1 * speed).rem_euclid(n as i32) as usize,
            );

            map[*r][*c] += 1;
            is_cloud[*r][*c] = true;
        }

        water_copy(&mut map, &clouds);

        let mut new_clouds = Vec::new();

        for r in 0..n {
            for c in 0..n {
                if map[r][c] < 2 || is_cloud[r][c] {
                    continue;
                }

                map[r][c] -= 2;
                new_clouds.push((r, c));
            }
        }

        for (r, c) in clouds {
            is_cloud[r][c] = false;
        }

        clouds = new_clouds;
    }

    map.iter().flatten().sum()
}

fn water_copy(map: &mut Vec<Vec<i32>>, clouds: &[(usize, usize)]) {
    let n = map.len() as i32;
    let mut waters = Vec::with_capacity(clouds.len());

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
            waters.push(((r, c), water));
        }
    }

    for ((r, c), water) in waters {
        map[r][c] += water;
    }
}
