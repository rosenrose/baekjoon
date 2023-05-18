use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);
    let mut input = || input.next().unwrap();

    let (n, min, max) = (input() as usize, input(), input());
    let map: Vec<Vec<_>> = (0..n).map(|_| (0..n).map(|_| input()).collect()).collect();

    let days = simulate(map, min, max);

    println!("{days}");
}

fn simulate(mut map: Vec<Vec<u32>>, min: u32, max: u32) -> u32 {
    let n = map.len();
    let mut days = 0;

    loop {
        let mut moves = Vec::new();
        let mut visited = vec![vec![false; n]; n];

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

                    for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
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
                moves.push((countries, sum));
            }
        }

        if moves.is_empty() {
            return days;
        }

        for (countries, sum) in moves {
            let population = sum / countries.len() as u32;

            for (r, c) in countries {
                map[r][c] = population;
            }
        }

        days += 1;
    }
}
