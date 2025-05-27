use std::io;

const MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let [n, min, max] = [(); 3].map(|_| input.next().unwrap());
    let mut map = [[0; MAX]; MAX];

    for r in 0..n as usize {
        for (c, num) in input.by_ref().take(n as usize).enumerate() {
            map[r][c] = num;
        }
    }

    let days = simulate(&mut map[..n as usize], min, max);

    println!("{days}");
}

fn simulate(map: &mut [[u32; MAX]], min: u32, max: u32) -> u32 {
    let n = map.len();
    let mut days = 0;

    loop {
        let mut moves = [(); MAX * MAX / 2].map(|_| (Vec::new(), 0));
        let mut moves_len = 0;
        let mut visited = [[false; MAX]; MAX];

        for y in 0..n {
            for x in 0..n {
                if visited[y][x] {
                    continue;
                }

                visited[y][x] = true;
                let mut countries = vec![(y, x)];
                let mut stack = vec![(y, x)];

                while let Some((r, c)) = stack.pop() {
                    let adjacents = [
                        (r.saturating_sub(1), c),
                        (r, c.saturating_sub(1)),
                        ((r + 1).min(n - 1), c),
                        (r, (c + 1).min(n - 1)),
                    ];

                    for (adj_r, adj_c) in adjacents {
                        if visited[adj_r][adj_c] {
                            continue;
                        }

                        let diff = map[r][c].abs_diff(map[adj_r][adj_c]);

                        if !(min <= diff && diff <= max) {
                            continue;
                        }

                        visited[adj_r][adj_c] = true;
                        countries.push((adj_r, adj_c));
                        stack.push((adj_r, adj_c));
                    }
                }

                if countries.len() < 2 {
                    continue;
                }

                let sum: u32 = countries.iter().map(|&(r, c)| map[r][c]).sum();

                moves[moves_len] = (countries, sum);
                moves_len += 1;
            }
        }

        if moves_len == 0 {
            return days;
        }

        for (countries, sum) in &moves[..moves_len] {
            let population = sum / countries.len() as u32;

            for &(r, c) in countries {
                map[r][c] = population;
            }
        }

        days += 1;
    }
}
