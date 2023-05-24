use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let (height, width) = (input.next().unwrap(), input.next().unwrap());
    let map: Vec<Vec<_>> = (0..height)
        .map(|_| input.by_ref().take(width).map(|num| num as u8).collect())
        .collect();

    let time = simulate(map);

    println!("{time}");
}

fn simulate(mut map: Vec<Vec<u8>>) -> i32 {
    let (width, height) = (map[0].len(), map.len());
    let mut time = 0;

    let is_pass = |r: usize, c: usize, visited: &[Vec<bool>], map: &[Vec<u8>]| {
        visited[r][c] || map[r][c] == 0
    };

    loop {
        time += 1;
        melt_ice(&mut map);

        let mut visited = vec![vec![false; width]; height];
        let mut count = 0;

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if is_pass(y, x, &visited, &map) {
                    continue;
                }

                visited[y][x] = true;
                let mut stack = vec![(y, x)];

                while let Some((r, c)) = stack.pop() {
                    for (adj_r, adj_c) in get_adjacents(r, c) {
                        if is_pass(adj_r, adj_c, &visited, &map) {
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

fn melt_ice(map: &mut Vec<Vec<u8>>) {
    let (width, height) = (map[0].len(), map.len());
    let mut melted = vec![vec![0; width]; height];

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

    *map = melted;
}

fn get_adjacents(r: usize, c: usize) -> [(usize, usize); 4] {
    [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)]
}
