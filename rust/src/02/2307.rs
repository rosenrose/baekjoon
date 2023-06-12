use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for [a, b, t] in (0..m).map(|_| [(); 3].map(|_| input())) {
        adjacency_list[a].push((b, t as i32));
        adjacency_list[b].push((a, t as i32));
    }

    let (start, end) = (1, n);
    let (times, prevs) = dijkstra_with_path(&adjacency_list, start);
    let escape_time = times[end];

    let mut max_delay_time = 0;
    let mut node = end;

    while node != start {
        let parent = prevs[node];
        let delay_time = dijkstra_except_edge(&adjacency_list, start, (parent, node))[end];

        max_delay_time = delay_time.max(max_delay_time);
        node = parent;
    }

    println!(
        "{}",
        if max_delay_time == i32::MAX {
            -1
        } else {
            max_delay_time - escape_time
        }
    );
}

fn dijkstra_with_path(graph: &[Vec<(usize, i32)>], start: usize) -> (Vec<i32>, Vec<usize>) {
    let mut dists = vec![i32::MAX; graph.len()];
    dists[start] = 0;

    let mut prevs = vec![0; graph.len()];
    let mut queue = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = dists[node];

        if dist > min_dist {
            continue;
        }

        for &(adj, weight) in &graph[node] {
            let adj_min_dist = dists[adj];
            let new_dist = min_dist + weight;

            if new_dist >= adj_min_dist {
                continue;
            }

            dists[adj] = new_dist;
            prevs[adj] = node;

            queue.push(Reverse((new_dist, adj)));
        }
    }

    (dists, prevs)
}

fn dijkstra_except_edge(
    graph: &[Vec<(usize, i32)>],
    start: usize,
    blocked_edge: (usize, usize),
) -> Vec<i32> {
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
