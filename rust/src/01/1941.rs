use std::io;

const SIZE: usize = 5;
const PARTY_COUNT: usize = 7;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut selected = [[false; SIZE]; SIZE];
    let (mut dasoms, mut doyeons) = ([(0, 0); SIZE * SIZE], [(0, 0); SIZE * SIZE]);
    let (mut dasoms_len, mut doyeons_len) = (0, 0);

    for (r, row) in buf.lines().enumerate() {
        for (c, ch) in row.char_indices() {
            match ch {
                'S' => {
                    dasoms[dasoms_len] = (r, c);
                    dasoms_len += 1
                }
                'Y' => {
                    doyeons[doyeons_len] = (r, c);
                    doyeons_len += 1
                }
                _ => unreachable!(),
            }
        }
    }

    let count: i32 = (0..=3.min(doyeons_len))
        .map(|i| {
            combinations_doyeons(
                0,
                0,
                i,
                &mut selected,
                &doyeons[..doyeons_len],
                &dasoms[..dasoms_len],
            )
        })
        .sum();

    println!("{count}");
}

fn combinations_doyeons(
    depth: usize,
    start: usize,
    max_depth: usize,
    selected: &mut [[bool; SIZE]],
    doyeons: &[(usize, usize)],
    dasoms: &[(usize, usize)],
) -> i32 {
    if depth == max_depth {
        return combinations_dasoms(
            0,
            0,
            (PARTY_COUNT - max_depth).min(dasoms.len()),
            selected,
            dasoms,
            Default::default(),
        );
    }

    let takes = doyeons.len() - (max_depth - 1);

    (start..depth + takes)
        .map(|i| {
            let (r, c) = doyeons[i];
            selected[r][c] = true;

            let result =
                combinations_doyeons(depth + 1, i + 1, max_depth, selected, doyeons, dasoms);
            selected[r][c] = false;

            result
        })
        .sum()
}

fn combinations_dasoms(
    depth: usize,
    start: usize,
    max_depth: usize,
    selected: &mut [[bool; SIZE]],
    dasoms: &[(usize, usize)],
    last_dasom: (usize, usize),
) -> i32 {
    if depth == max_depth {
        let start = last_dasom;
        let mut count = 0;
        let mut visited = [[false; SIZE]; SIZE];
        visited[start.0][start.1] = true;

        let mut stack = vec![start];

        while let Some((r, c)) = stack.pop() {
            count += 1;

            let adjacents = [
                (r.saturating_sub(1), c),
                (r, c.saturating_sub(1)),
                ((r + 1).min(SIZE - 1), c),
                (r, (c + 1).min(SIZE - 1)),
            ];

            for (adj_r, adj_c) in adjacents {
                if visited[adj_r][adj_c] || !selected[adj_r][adj_c] {
                    continue;
                }

                visited[adj_r][adj_c] = true;
                stack.push((adj_r, adj_c));
            }
        }

        return (count == PARTY_COUNT) as i32;
    }

    let takes = dasoms.len() - (max_depth - 1);

    (start..depth + takes)
        .map(|i| {
            let (r, c) = dasoms[i];
            selected[r][c] = true;

            let result =
                combinations_dasoms(depth + 1, i + 1, max_depth, selected, dasoms, dasoms[i]);
            selected[r][c] = false;

            result
        })
        .sum()
}
