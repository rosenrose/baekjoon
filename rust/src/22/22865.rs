use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let [a, b, c] = [(); 3].map(|_| input());
    let m = input();

    let mut adjacency_array = (vec![i32::MAX; n + 1], vec![((0, 0), 0); m << 1]);

    for (i, [d, e, len]) in (0..m).map(|i| (i << 1, [(); 3].map(|_| input()))) {
        let prev = adjacency_array.0[d];
        adjacency_array.0[d] = i as i32;
        adjacency_array.1[i] = ((e as i32, len as i32), prev);

        let prev = adjacency_array.0[e];
        adjacency_array.0[e] = (i + 1) as i32;
        adjacency_array.1[i + 1] = ((d as i32, len as i32), prev);
    }

    let dists_from_a = dijkstra(&adjacency_array, a);
    let dists_from_b = dijkstra(&adjacency_array, b);
    let dists_from_c = dijkstra(&adjacency_array, c);
    let mut max_dist = 0;
    let mut most_far = 0;

    for node in 1..=n {
        let min_dist = *[dists_from_a[node], dists_from_b[node], dists_from_c[node]]
            .iter()
            .filter(|&&dist| dist != 0)
            .min()
            .unwrap();

        if min_dist > max_dist {
            max_dist = min_dist;
            most_far = node;
        }
    }

    println!("{most_far}");
}

fn dijkstra((nodes, edges): &(Vec<i32>, Vec<((i32, i32), i32)>), start: usize) -> Vec<i32> {
    let mut dists = vec![i32::MAX; nodes.len()];
    dists[start] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start as i32))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = dists[node as usize];
        let mut edge = nodes[node as usize];

        if dist > min_dist {
            continue;
        }

        while let Some(&((adj, weight), next_edge)) = edges.get(edge as usize) {
            let adj_min_dist = dists[adj as usize];
            let new_dist = min_dist + weight;

            if new_dist < adj_min_dist {
                dists[adj as usize] = new_dist;
                queue.push(Reverse((new_dist, adj)));
            }

            edge = next_edge;
        }
    }

    dists
}
