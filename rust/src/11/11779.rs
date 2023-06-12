use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input() as usize, input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for [u, v, w] in (0..m).map(|_| [(); 3].map(|_| input())) {
        adjacency_list[u as usize].push((v, w));
    }

    let (start, mut end) = (input() as usize, input() as usize);
    let (distances, prevs) = dijkstra_with_path(&adjacency_list, start);

    println!("{}", distances[end]);

    let mut path = vec![end];

    while end != start {
        end = prevs[end] as usize;
        path.push(end);
    }

    println!("{}", path.len());

    for p in path.iter().rev() {
        print!("{p} ");
    }
}

fn dijkstra_with_path(graph: &[Vec<(i32, i32)>], start: usize) -> (Vec<i32>, Vec<i32>) {
    let mut distances = vec![i32::MAX; graph.len()];
    distances[start] = 0;

    let mut prevs = vec![0; graph.len()];
    let mut queue = BinaryHeap::from([Reverse((0, start as i32))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = distances[node as usize];

        if dist > min_dist {
            continue;
        }

        for &(adj, weight) in &graph[node as usize] {
            let adj_min_dist = distances[adj as usize];
            let new_dist = min_dist + weight;

            if new_dist >= adj_min_dist {
                continue;
            }

            distances[adj as usize] = new_dist;
            prevs[adj as usize] = node;

            queue.push(Reverse((new_dist, adj)));
        }
    }

    (distances, prevs)
}
