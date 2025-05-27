use std::io;

const SIZE_MAX: usize = 200;
const VIRUS_MAX: usize = SIZE_MAX * SIZE_MAX;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, _k) = (input(), input());
    let mut viruses = [(0, (0, 0)); VIRUS_MAX];
    let mut viruses_len = 0;
    let mut visited = [[false; SIZE_MAX]; SIZE_MAX];
    let mut map = [[0; SIZE_MAX]; SIZE_MAX];

    for r in 0..n {
        for c in 0..n {
            let num = input();

            if num > 0 {
                viruses[viruses_len] = (num, (r, c));
                viruses_len += 1;
                visited[r][c] = true;
            }

            map[r][c] = num;
        }
    }

    let (s, target) = (input(), (input() - 1, input() - 1));

    viruses[..viruses_len].sort_unstable();
    let mut temp = [(0, (0, 0)); VIRUS_MAX];
    let mut temp_len;

    for _ in 0..s {
        temp[..viruses_len].copy_from_slice(&viruses[..viruses_len]);
        temp_len = viruses_len;
        viruses_len = 0;

        for &(virus, (r, c)) in &temp[..temp_len] {
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

                viruses[viruses_len] = (virus, (adj_r, adj_c));
                viruses_len += 1;
            }
        }
    }

    println!("{}", map[target.0][target.1]);
}
