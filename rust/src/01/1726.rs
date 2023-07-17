use std::collections::VecDeque;
use std::io;

const DIRS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i8>);
    let mut input = || input.next().unwrap();

    let (height, width) = (input(), input());
    let map: Vec<Vec<_>> = (0..height)
        .map(|_| (0..width).map(|_| input() == 1).collect())
        .collect();
    let start = [(); 3].map(|_| input() - 1);
    let end = [(); 3].map(|_| input() - 1);

    let mut visited = vec![vec![[false; DIRS.len()]; width as usize]; height as usize];
    visited[start[0] as usize][start[1] as usize][start[2] as usize] = true;

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some(([r, c, dir], count)) = queue.pop_front() {
        if [r, c, dir] == end {
            println!("{count}");
            return;
        }

        let go = (1..=3).map_while(|i| {
            let (moved_r, moved_c) = (r + DIRS[dir as usize].0 * i, c + DIRS[dir as usize].1 * i);

            (0 <= moved_r
                && moved_r < height
                && 0 <= moved_c
                && moved_c < width
                && !map[moved_r as usize][moved_c as usize])
                .then_some([moved_r, moved_c, dir])
        });
        let turn = match dir {
            0 | 1 => [2, 3],
            2 | 3 => [0, 1],
            _ => unreachable!(),
        }
        .map(|turned_dir| [r, c, turned_dir]);

        for adj in go.chain(turn) {
            if visited[adj[0] as usize][adj[1] as usize][adj[2] as usize] {
                continue;
            }

            visited[adj[0] as usize][adj[1] as usize][adj[2] as usize] = true;
            queue.push_back((adj, count + 1));
        }
    }
}
