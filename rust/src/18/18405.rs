use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, _) = (input(), input());
    let mut viruses = Vec::new();
    let mut visited = vec![vec![false; n]; n];
    let mut map: Vec<Vec<_>> = (0..n)
        .map(|r| {
            (0..n)
                .map(|c| {
                    let num = input();

                    if num > 0 {
                        viruses.push((num, (r, c)));
                        visited[r][c] = true;
                    }

                    num
                })
                .collect()
        })
        .collect();
    let (s, target) = (input(), (input() - 1, input() - 1));

    viruses.sort_unstable();
    let mut temp = Vec::new();

    for _ in 0..s {
        temp.clone_from(&viruses);
        viruses.clear();

        for &(virus, (r, c)) in &temp {
            let adjacents = [
                (r.saturating_sub(1), c),
                (r, c.saturating_sub(1)),
                ((r + 1).min(n - 1), c),
                (r, (c + 1).min(n - 1)),
            ];

            for (adj_r, adj_c) in adjacents {
                let adj_virus = map[adj_r][adj_c];

                if visited[adj_r][adj_c] || 0 < adj_virus && adj_virus <= virus {
                    continue;
                }

                visited[adj_r][adj_c] = true;
                map[adj_r][adj_c] = virus;
                viruses.push((virus, (adj_r, adj_c)));
            }
        }
    }

    println!("{}", map[target.0][target.1]);
}
