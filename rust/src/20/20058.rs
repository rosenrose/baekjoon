use std::io;

const MAX: usize = 64;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, _q] = [(); 2].map(|_| input.next().unwrap());
    let size = 1 << n as usize;
    let mut map = [[0; MAX]; MAX];

    for r in 0..size {
        for (c, num) in input.by_ref().take(size).enumerate() {
            map[r][c] = num;
        }
    }

    let (sum, max_count) = simulate(&mut map[..size], input);

    println!("{sum}\n{max_count}");
}

fn simulate(map: &mut [[i32; MAX]], steps: impl Iterator<Item = i32>) -> (i32, i32) {
    let size = map.len();
    let (mut sum, mut max_count) = (0, 0);

    for step in steps {
        rotate(map, 1 << step as usize);
        melt_ice(map);
        // for r in &map {
        //     println!("{r:?}");
        // }
    }

    let mut visited = [[false; MAX]; MAX];
    let is_pass = |r: usize, c: usize, visited: &[[bool; MAX]]| visited[r][c] || map[r][c] == 0;

    for y in 0..size {
        for x in 0..size {
            if is_pass(y, x, &visited) {
                continue;
            }

            visited[y][x] = true;
            let mut count = 0;
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                sum += map[r][c];
                count += 1;

                for (adj_r, adj_c) in get_adjacents(r, c, size) {
                    if is_pass(adj_r, adj_c, &visited) {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            max_count = count.max(max_count);
        }
    }

    (sum, max_count)
}

fn get_adjacents(r: usize, c: usize, size: usize) -> [(usize, usize); 4] {
    [
        (r.saturating_sub(1), c),
        (r, c.saturating_sub(1)),
        ((r + 1).min(size - 1), c),
        (r, (c + 1).min(size - 1)),
    ]
}

fn rotate(map: &mut [[i32; MAX]], inner_size: usize) {
    if inner_size == 1 {
        return;
    }

    let size = map.len();
    let mut rotated = [[0; MAX]; MAX];

    for y in (0..size).step_by(inner_size) {
        for x in (0..size).step_by(inner_size) {
            for c in 0..inner_size {
                for r in (0..inner_size).rev() {
                    rotated[y + c][x + (inner_size - r - 1)] = map[y + r][x + c];
                }
            }
        }
    }

    for (r, row) in map.iter_mut().enumerate() {
        row[..size].copy_from_slice(&rotated[r][..size]);
    }
}

fn melt_ice(map: &mut [[i32; MAX]]) {
    let size = map.len();
    let mut melts = [(0, 0); MAX * MAX];
    let mut melts_len = 0;

    for r in 0..size {
        for c in 0..size {
            if map[r][c] == 0 {
                continue;
            }

            let ice_count = get_adjacents(r, c, size)
                .iter()
                .filter(|&&(adj_r, adj_c)| (adj_r, adj_c) != (r, c) && map[adj_r][adj_c] > 0)
                .count();

            if ice_count < 3 {
                melts[melts_len] = (r, c);
                melts_len += 1;
            }
        }
    }

    for &(r, c) in &melts[..melts_len] {
        map[r][c] -= 1;
    }
}
