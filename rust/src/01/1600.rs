use std::collections::VecDeque;
use std::io;

const WIDTH_MAX: usize = 200;
const HEIGHT_MAX: usize = 200;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let k = input.next().unwrap() as u8;
    let [width, height] = [(); 2].map(|_| input.next().unwrap());
    let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for r in 0..height {
        for (c, num) in input.by_ref().take(width as usize).enumerate() {
            map[r as usize][c] = num == 1;
        }
    }

    let mut visited = [[[false; 31]; WIDTH_MAX]; HEIGHT_MAX];
    visited[0][0][0] = true;

    let mut queue = VecDeque::from([((0, 0), 0, 0)]);

    while let Some(((r, c), dist, jump_count)) = queue.pop_front() {
        if (r, c) == (height - 1, width - 1) {
            println!("{dist}");
            return;
        }

        let new_dist = dist + 1;
        let new_jump_count = jump_count + 1;
        let monkey_adjacents = [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)];
        let horse_adjacents = [
            (r - 1, c - 2),
            (r - 2, c - 1),
            (r - 2, c + 1),
            (r - 1, c + 2),
            (r + 1, c - 2),
            (r + 2, c - 1),
            (r + 2, c + 1),
            (r + 1, c + 2),
        ];

        for (i, (adj_r, adj_c)) in monkey_adjacents
            .into_iter()
            .chain(horse_adjacents)
            .enumerate()
            .filter(|&(_, (adj_r, adj_c))| {
                0 <= adj_r && adj_r < height && 0 <= adj_c && adj_c < width
            })
        {
            let adj = (adj_r as usize, adj_c as usize);

            if map[adj.0][adj.1] {
                continue;
            }

            if i < monkey_adjacents.len() {
                if visited[adj.0][adj.1][jump_count as usize] {
                    continue;
                }

                visited[adj.0][adj.1][jump_count as usize] = true;
                queue.push_back(((adj_r, adj_c), new_dist, jump_count));

                continue;
            }

            if jump_count < k {
                if visited[adj.0][adj.1][new_jump_count as usize] {
                    continue;
                }

                for j in (new_jump_count..=k).step_by(2) {
                    visited[adj.0][adj.1][j as usize] = true;
                }

                queue.push_back(((adj_r, adj_c), new_dist, new_jump_count));
            }
        }
    }

    println!("-1");
}
