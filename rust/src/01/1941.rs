use std::io;

const SIZE: usize = 5;
const PARTY_COUNT: usize = 7;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut selected = [[false; SIZE]; SIZE];
    let mut dasom = Vec::new();
    let mut doyeon = Vec::new();

    for (r, row) in buf.lines().enumerate() {
        for (c, ch) in row.char_indices() {
            match ch {
                'S' => dasom.push((r, c)),
                'Y' => doyeon.push((r, c)),
                _ => unreachable!(),
            }
        }
    }

    let count: i32 = (0..=3.min(doyeon.len()))
        .map(|i| combinations_doyeon(0, 0, i, &mut selected, &doyeon, &dasom))
        .sum();

    println!("{count}");
}

fn combinations_doyeon(
    depth: usize,
    start: usize,
    max_depth: usize,
    selected: &mut [[bool; SIZE]],
    doyeon: &[(usize, usize)],
    dasom: &[(usize, usize)],
) -> i32 {
    if depth == max_depth {
        return combinations_dasom(
            0,
            0,
            (PARTY_COUNT - max_depth).min(dasom.len()),
            selected,
            dasom,
            Default::default(),
        );
    }

    let takes = doyeon.len() - (max_depth - 1);

    (start..depth + takes)
        .map(|i| {
            let (r, c) = doyeon[i];
            selected[r][c] = true;

            let result = combinations_doyeon(depth + 1, i + 1, max_depth, selected, doyeon, dasom);
            selected[r][c] = false;

            result
        })
        .sum()
}

fn combinations_dasom(
    depth: usize,
    start: usize,
    max_depth: usize,
    selected: &mut [[bool; SIZE]],
    dasom: &[(usize, usize)],
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

        return i32::from(count == PARTY_COUNT);
    }

    let takes = dasom.len() - (max_depth - 1);

    (start..depth + takes)
        .map(|i| {
            let (r, c) = dasom[i];
            selected[r][c] = true;

            let result = combinations_dasom(depth + 1, i + 1, max_depth, selected, dasom, dasom[i]);
            selected[r][c] = false;

            result
        })
        .sum()
}
