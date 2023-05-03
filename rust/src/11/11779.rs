use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (u, v, w) in (0..m).map(|_| (input(), input(), input() as i32)) {
        adjacency_list[u].push((v, w));
    }

    let (start, mut end) = (input(), input());
    let (distances, prevs) = dijkstra_with_path(&adjacency_list, start);

    println!("{}", distances[end]);

    let mut path = vec![end];

    while end != start {
        end = prevs[end];
        path.push(end);
    }

    println!("{}", path.len());

    for p in path.iter().rev() {
        print!("{p} ");
    }
}

fn dijkstra_with_path(graph: &[Vec<(usize, i32)>], start: usize) -> (Vec<i32>, Vec<usize>) {
    let mut distances = vec![i32::MAX; graph.len()];
    distances[start] = 0;

    let mut prevs = vec![0; graph.len()];
    let mut queue = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = distances[node];

        if dist > min_dist {
            continue;
        }

        for &(neighbor, weight) in graph[node].iter() {
            let neighbor_min_dist = distances[neighbor];
            let new_dist = min_dist + weight;

            if new_dist >= neighbor_min_dist {
                continue;
            }

            distances[neighbor] = new_dist;
            prevs[neighbor] = node;

            queue.push(Reverse((new_dist, neighbor)));
        }
    }

    (distances, prevs)
}
