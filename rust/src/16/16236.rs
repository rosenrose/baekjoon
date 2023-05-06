use std::collections::VecDeque;
use std::io;

enum Cells {
    Empty,
    Fish(i32),
    Shark,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n: usize = input.next().unwrap() as usize;
    let mut shark_coord = (0, 0);
    let (mut shark_size, mut shark_eats) = (2, 0);

    let mut map: Vec<Vec<_>> = (0..n)
        .map(|r| {
            input
                .by_ref()
                .take(n)
                .enumerate()
                .map(|(c, num)| match num {
                    0 => Cells::Empty,
                    1..=6 => Cells::Fish(num),
                    9 => {
                        shark_coord = (r, c);
                        Cells::Shark
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let mut time = 0;

    loop {
        let (target, cost) = eat(&map, shark_coord, shark_size);

        if cost == i32::MAX {
            break;
        }

        map[shark_coord.0][shark_coord.1] = Cells::Empty;
        map[target.0][target.1] = Cells::Shark;

        shark_coord = target;
        shark_eats += 1;

        if shark_eats == shark_size {
            shark_size += 1;
            shark_eats = 0;
        }

        time += cost;
    }

    println!("{time}");
}

fn eat(map: &[Vec<Cells>], shark_coord: (usize, usize), shark_size: i32) -> ((usize, usize), i32) {
    let n = map.len();
    let mut visited = vec![vec![false; n]; n];
    visited[shark_coord.0][shark_coord.1] = true;

    let mut queue = VecDeque::from([(shark_coord, 0)]);
    let (mut target, mut cost) = ((0, 0), i32::MAX);

    while let Some(((r, c), time)) = queue.pop_front() {
        if matches!(map[r][c], Cells::Fish(s) if s < shark_size) {
            if (time, (r, c)) < (cost, target) {
                cost = time;
                target = (r, c);
            }
        }

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(n - 1), c),
            (r, (c + 1).min(n - 1)),
        ];

        for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
            if visited[adj_r][adj_c]
                || matches!(map[adj_r][adj_c], Cells::Fish(s) if s > shark_size)
            {
                continue;
            }

            visited[adj_r][adj_c] = true;
            queue.push_back(((adj_r, adj_c), time + 1));
        }
    }

    (target, cost)
}
