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
    let mut adjacency_array = (vec![i32::MAX; n + 1], vec![(0, 0); m]);

    for (i, (u, v)) in (0..m).map(|i| (i, (input(), input()))) {
        let prev = adjacency_array.0[u];

        adjacency_array.0[u] = i as i32;
        adjacency_array.1[i] = (v as i32, prev);
    }

    let distances = dijkstra(&adjacency_array, x, k);
    let mut result: Vec<_> = distances
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(node, &d)| (d == k).then_some(node))
        .collect();

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

fn dijkstra(
    (nodes, edges): &(Vec<i32>, Vec<(i32, i32)>),
    start: usize,
    target_dist: i32,
) -> Vec<i32> {
    let mut distances = vec![i32::MAX; nodes.len()];
    distances[start] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start as i32))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = distances[node as usize];
        let mut edge = nodes[node as usize];

        if dist > min_dist || dist > target_dist {
            continue;
        }
        if edge == i32::MAX {
            continue;
        }

        loop {
            let (neighbor, next_edge) = edges[edge as usize];
            let neighbor_min_dist = distances[neighbor as usize];
            let new_dist = min_dist + 1;

            if new_dist < neighbor_min_dist && new_dist <= target_dist {
                distances[neighbor as usize] = new_dist;
                queue.push(Reverse((new_dist, neighbor)));
            }

            if next_edge == i32::MAX {
                break;
            }

            edge = next_edge;
        }
    }

    distances
}
