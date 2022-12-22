use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());

    let adjacency_list = (0..m).fold(vec![Vec::new(); n + 1], |mut acc, _| {
        let (u, v, w) = (input(), input(), input());
        acc[u].push((v, w as i32));

        acc
    });

    let (start, mut end) = (input(), input());
    let (distances, prevs) = dijkstra(&adjacency_list, start);

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

fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> (Vec<i32>, Vec<usize>) {
    let mut distances = vec![i32::MAX; graph.len()];
    distances[start] = 0;

    let mut prevs = vec![0; graph.len()];
    let mut candidates = BinaryHeap::from([Reverse((0, start))]);

    while !candidates.is_empty() {
        let (dist, node) = candidates.pop().unwrap().0;
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

            let new_candi = (new_dist, neighbor);
            candidates.push(Reverse(new_candi));
        }
    }

    (distances, prevs)
}
