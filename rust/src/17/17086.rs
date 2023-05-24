use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let (height, width) = (input.next().unwrap(), input.next().unwrap());
    let mut queue = VecDeque::with_capacity(height * width);
    let mut safe_dists = vec![vec![0; width]; height];

    for r in 0..height {
        for (c, num) in input.by_ref().take(width).enumerate() {
            if num == 1 {
                queue.push_back(((r, c), 0));
                safe_dists[r][c] = 1;
            }
        }
    }

    while let Some(((row, col), dist)) = queue.pop_front() {
        let (up, down, left, right) = (
            row.saturating_sub(1),
            (row + 1).min(height - 1),
            col.saturating_sub(1),
            (col + 1).min(width - 1),
        );
        let adjacents = [
            (up, left),
            (up, col),
            (up, right),
            (row, left),
            (row, right),
            (down, left),
            (down, col),
            (down, right),
        ];

        for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (row, col)) {
            if safe_dists[adj_r][adj_c] > 0 {
                continue;
            }

            safe_dists[adj_r][adj_c] = dist + 1;
            queue.push_back(((adj_r, adj_c), dist + 1));
        }
    }
    // for r in &safe_dists {
    //     println!("{r:?}");
    // }
    println!("{}", safe_dists.iter().flatten().max().unwrap());
}
