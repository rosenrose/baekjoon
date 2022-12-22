use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (v, e, k) = (input(), input(), input());

    let adjacency_list = (0..e).fold(vec![Vec::new(); v + 1], |mut acc, _| {
        let (u, v, w) = (input(), input(), input());
        acc[u].push((v, w as i32));

        acc
    });

    let (distances, prevs) = dijkstra(&adjacency_list, k);
    // println!("{distances:?} {prevs:?}");

    for &dist in distances.iter().skip(1) {
        (if dist == i32::MAX {
            writeln!(output, "INF")
        } else {
            writeln!(output, "{dist}")
        })
        .unwrap();
    }

    print!("{output}");
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
