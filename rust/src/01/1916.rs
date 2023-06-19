use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut adjacency_array = (vec![i32::MAX; n + 1], Vec::with_capacity(m));

    for [u, v, w] in (0..m).map(|_| [(); 3].map(|_| input())) {
        let prev = adjacency_array.0[u];

        adjacency_array.0[u] = adjacency_array.1.len() as i32;
        adjacency_array.1.push(((v as i32, w as i32), prev));
    }

    let (start, end) = (input() as i32, input() as i32);
    let distance = dijkstra(&adjacency_array, start, end);

    println!("{distance}");
}

fn dijkstra((nodes, edges): &(Vec<i32>, Vec<((i32, i32), i32)>), start: i32, end: i32) -> i32 {
    let mut distances = vec![i32::MAX; nodes.len()];
    distances[start as usize] = 0;

    let mut queue = BinaryHeap::from([(Reverse(0), start)]);

    while let Some((Reverse(dist), node)) = queue.pop() {
        let min_dist = distances[node as usize];

        if node == end {
            return min_dist;
        }
        if dist > min_dist {
            continue;
        }

        let mut edge = nodes[node as usize];

        while let Some(&((adj, weight), next_edge)) = edges.get(edge as usize) {
            let adj_min_dist = distances[adj as usize];
            let new_dist = min_dist + weight;

            if new_dist < adj_min_dist {
                distances[adj as usize] = new_dist;
                queue.push((Reverse(new_dist), adj));
            }

            edge = next_edge;
        }
    }

    distances[end as usize]
}
