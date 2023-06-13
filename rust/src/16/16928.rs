use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    const SIZE: usize = 10 * 10;
    let mut board: Vec<_> = (0..=SIZE).collect();
    let (n, m) = (input(), input());

    for (x, y) in (0..n).map(|_| (input(), input())) {
        board[x] = y
    }
    for (u, v) in (0..m).map(|_| (input(), input())) {
        board[u] = v;
    }

    let (start, end) = (1, SIZE);
    let mut visited = [false; SIZE + 1];
    visited[start] = true;

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((num, count)) = queue.pop_front() {
        let next_count = count + 1;
        let adjacents = (1..=6).flat_map(|i| board.get(num + i));

        for &adj in adjacents {
            if adj == end {
                println!("{next_count}");
                return;
            }

            if visited[adj] {
                continue;
            }

            visited[adj] = true;
            queue.push_back((adj, next_count));
        }
    }
}
