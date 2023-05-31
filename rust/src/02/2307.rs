use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];
    let mut edges = Vec::with_capacity(m);

    for (a, b, t) in (0..m).map(|_| (input(), input(), input() as i32)) {
        adjacency_list[a].push((b, t));
        adjacency_list[b].push((a, t));
        edges.push((a, b));
    }

    let (start, end) = (1, n);
    let escape_time = dijkstra(&adjacency_list, start, (0, 0))[end];
    let max_delayed_time = edges
        .iter()
        .map(|&edge| dijkstra(&adjacency_list, start, edge)[end])
        .max()
        .unwrap();

    println!(
        "{}",
        if max_delayed_time == i32::MAX {
            -1
        } else {
            max_delayed_time - escape_time
        }
    );
}

fn dijkstra(graph: &[Vec<(usize, i32)>], start: usize, blocked_edge: (usize, usize)) -> Vec<i32> {
    let mut dists = vec![i32::MAX; graph.len()];
    dists[start] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = dists[node];

        if dist > min_dist {
            continue;
        }

        for &(adj, weight) in &graph[node] {
            if (node, adj) == blocked_edge || (adj, node) == blocked_edge {
                continue;
            }

            let adj_min_dist = dists[adj];
            let new_dist = min_dist + weight;

            if new_dist >= adj_min_dist {
                continue;
            }

            dists[adj] = new_dist;
            queue.push(Reverse((new_dist, adj)));
        }
    }

    dists
}
