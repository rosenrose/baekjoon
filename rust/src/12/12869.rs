use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut healths = [0; 3];

    for (i, num) in input.skip(1).enumerate() {
        healths[i] = num;
    }

    const MAX: usize = 60;
    let mut visited = [[[false; MAX + 1]; MAX + 1]; MAX + 1];
    visited[healths[0]][healths[1]][healths[2]] = true;

    let mut queue = VecDeque::from([(healths, 0)]);

    while let Some((healths, count)) = queue.pop_front() {
        let next_count = count + 1;
        let adjacents = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ]
        .map(|[first, second, third]| {
            [
                healths[first].saturating_sub(9),
                healths[second].saturating_sub(3),
                healths[third].saturating_sub(1),
            ]
        });

        for adj in adjacents {
            if adj == [0; 3] {
                println!("{next_count}");
                return;
            }

            if visited[adj[0]][adj[1]][adj[2]] {
                continue;
            }

            visited[adj[0]][adj[1]][adj[2]] = true;
            queue.push_back((adj, next_count));
        }
    }
}
