use std::io;

enum Cells {
    Empty,
    Wall,
    Sheep,
    Wolf,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let map: Vec<Vec<_>> = input
        .take(height) // 입력 마지막 줄에 이상한 문자 들어있음
        .map(|row| {
            row.chars()
                .map(|ch| match ch {
                    '.' => Cells::Empty,
                    '#' => Cells::Wall,
                    'k' => Cells::Sheep,
                    'v' => Cells::Wolf,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let (mut total_sheep, mut total_wolf) = (0, 0);
    let mut visited = vec![vec![false; width]; height];

    let is_pass = |r: usize, c: usize, visited: &[Vec<bool>]| {
        visited[r][c] || matches!(map[r][c], Cells::Wall)
    };

    for y in 0..height {
        for x in 0..width {
            if is_pass(y, x, &visited) {
                continue;
            }

            visited[y][x] = true;
            let (mut sheep_count, mut wolf_count) = (0, 0);
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                match map[r][c] {
                    Cells::Sheep => sheep_count += 1,
                    Cells::Wolf => wolf_count += 1,
                    _ => (),
                }

                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(height - 1), c),
                    (r, (c + 1).min(width - 1)),
                ];

                for (adj_r, adj_c) in adjacents {
                    if is_pass(adj_r, adj_c, &visited) {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            if sheep_count > wolf_count {
                total_sheep += sheep_count;
            } else {
                total_wolf += wolf_count;
            }
        }
    }

    println!("{total_sheep} {total_wolf}");
}
