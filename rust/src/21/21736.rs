use std::io;

enum Cells {
    Empty,
    Wall,
    People,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut start = (0, 0);
    let map: Vec<Vec<_>> = input
        .enumerate()
        .map(|(r, row)| {
            row.char_indices()
                .map(|(c, ch)| match ch {
                    'O' => Cells::Empty,
                    'X' => Cells::Wall,
                    'I' => {
                        start = (r, c);
                        Cells::Empty
                    }
                    'P' => Cells::People,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut count = 0;
    let mut visited = vec![vec![false; width]; height];
    visited[start.0][start.1] = true;

    let mut stack = vec![start];

    while let Some((r, c)) = stack.pop() {
        if let Cells::People = map[r][c] {
            count += 1;
        }

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if visited[adj_r][adj_c] || matches!(map[adj_r][adj_c], Cells::Wall) {
                continue;
            }

            visited[adj_r][adj_c] = true;
            stack.push((adj_r, adj_c));
        }
    }

    if count == 0 {
        println!("TT");
        return;
    }

    println!("{count}");
}
