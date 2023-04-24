use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m, k, x) = (input(), input(), input() as i32, input());
    let mut adjacency_array = (vec![usize::MAX; n + 1], vec![(0, 0); m]);

    for (i, (u, v)) in (0..m).map(|i| (i, (input(), input()))) {
        let prev = adjacency_array.0[u];

        adjacency_array.0[u] = i;
        adjacency_array.1[i] = (v, prev);
    }

    let distances = dijkstra(&adjacency_array, x, k);
    let mut result: Vec<_> = distances
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, &d)| d == k)
        .collect();

    if result.is_empty() {
        println!("-1");
        return;
    }

    result.sort_unstable();

    for (node, _) in result {
        writeln!(output, "{node}").unwrap();
    }

    print!("{output}");
}

fn dijkstra(
    (nodes, edges): &(Vec<usize>, Vec<(usize, usize)>),
    start: usize,
    target_dist: i32,
) -> Vec<i32> {
    let mut distances = vec![i32::MAX; nodes.len()];
    distances[start] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = distances[node];
        let mut edge = nodes[node];

        if dist > min_dist || dist > target_dist {
            continue;
        }
        if edge == usize::MAX {
            continue;
        }

        loop {
            let (neighbor, next_edge) = edges[edge];
            let neighbor_min_dist = distances[neighbor];
            let new_dist = min_dist + 1;

            if new_dist < neighbor_min_dist && new_dist <= target_dist {
                distances[neighbor] = new_dist;
                queue.push(Reverse((new_dist, neighbor)));
            }

            if next_edge == usize::MAX {
                break;
            }

            edge = next_edge;
        }
    }

    distances
}
