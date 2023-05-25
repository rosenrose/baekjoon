use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m, k, x) = (input() as usize, input() as usize, input(), input());
    let mut adjacency_array = (vec![i32::MAX; n + 1], vec![(0, 0); m]);

    for (i, (u, v)) in (0..m).map(|i| (i, (input() as usize, input()))) {
        let prev = adjacency_array.0[u];

        adjacency_array.0[u] = i as i32;
        adjacency_array.1[i] = (v, prev);
    }

    let mut result = bfs(&adjacency_array, x, k);

    if result.is_empty() {
        println!("-1");
        return;
    }

    result.sort_unstable();

    for node in result {
        writeln!(output, "{node}").unwrap();
    }

    print!("{output}");
}

fn bfs((nodes, edges): &(Vec<i32>, Vec<(i32, i32)>), start: i32, target_dist: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut visited = vec![false; nodes.len()];
    let mut queue = VecDeque::from([(start, 0)]);
    visited[start as usize] = true;

    while let Some((node, dist)) = queue.pop_front() {
        if dist == target_dist {
            result.push(node);
            continue;
        }

        let mut edge = nodes[node as usize];

        if edge == i32::MAX {
            continue;
        }

        loop {
            let (adj, next_edge) = edges[edge as usize];

            if !visited[adj as usize] {
                visited[adj as usize] = true;

                if dist < target_dist {
                    queue.push_back((adj, dist + 1));
                }
            }

            if next_edge == i32::MAX {
                break;
            }

            edge = next_edge;
        }
    }

    result
}
