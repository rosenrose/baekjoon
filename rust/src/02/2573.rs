use std::io;

const WIDTH_MAX: usize = 300;
const HEIGHT_MAX: usize = 300;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width] = [(); 2].map(|_| input.next().unwrap());
    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for r in 0..height {
        for (c, num) in input.by_ref().take(width).enumerate() {
            map[r][c] = num as u8;
        }
    }

    let time = simulate(&mut map[..height], width);

    println!("{time}");
}

fn simulate(map: &mut [[u8; WIDTH_MAX]], width: usize) -> i32 {
    let height = map.len();
    let mut time = 0;

    let is_pass = |r: usize, c: usize, visited: &[[bool; WIDTH_MAX]], map: &[[u8; WIDTH_MAX]]| {
        visited[r][c] || map[r][c] == 0
    };

    loop {
        time += 1;
        melt_ice(map, width);

        let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
        let mut count = 0;

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if is_pass(y, x, &visited, map) {
                    continue;
                }

                visited[y][x] = true;
                let mut stack = vec![(y, x)];

                while let Some((r, c)) = stack.pop() {
                    for (adj_r, adj_c) in get_adjacents(r, c) {
                        if is_pass(adj_r, adj_c, &visited, map) {
                            continue;
                        }

                        visited[adj_r][adj_c] = true;
                        stack.push((adj_r, adj_c));
                    }
                }

                count += 1;

                if count > 1 {
                    return time;
                }
            }
        }

        if count == 0 {
            return 0;
        }
    }
}

fn melt_ice(map: &mut [[u8; WIDTH_MAX]], width: usize) {
    let height = map.len();
    let mut melted = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for r in 1..height - 1 {
        for c in 1..width - 1 {
            if map[r][c] == 0 {
                continue;
            }

            let water_count = get_adjacents(r, c)
                .iter()
                .filter(|&&(adj_r, adj_c)| map[adj_r][adj_c] == 0)
                .count() as u8;

            melted[r][c] = map[r][c].saturating_sub(water_count);
        }
    }

    for (r, row) in map.iter_mut().enumerate() {
        row[..width].copy_from_slice(&melted[r][..width]);
    }
}

fn get_adjacents(r: usize, c: usize) -> [(usize, usize); 4] {
    [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)]
}
