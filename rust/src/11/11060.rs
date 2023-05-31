use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let map: Vec<_> = input.skip(1).collect();
    const MAX: usize = 1000;

    let mut visited = [false; MAX];
    visited[0] = true;

    let mut queue = VecDeque::from([(0, 0)]);

    while let Some((num, jumps)) = queue.pop_front() {
        if num == map.len() - 1 {
            println!("{jumps}");
            return;
        }

        for adj in num + 1..=num + map[num] {
            if adj >= MAX || visited[adj] {
                continue;
            }

            visited[adj] = true;
            queue.push_back((adj, jumps + 1));
        }
    }

    println!("-1");
}
