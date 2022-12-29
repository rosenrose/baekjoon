use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m, k, x) = (input() as usize, input(), input(), input());

    let adjacency_list = (0..m).fold(vec![Vec::new(); n + 1], |mut acc, _| {
        let (u, v) = (input() as usize, input());
        acc[u].push(v);

        acc
    });

    let distances = dijkstra(&adjacency_list, x, k);
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

    result.sort_unstable_by_key(|&(node, _)| node);

    for (node, _) in result {
        writeln!(output, "{node}").unwrap();
    }

    print!("{output}");
}

fn dijkstra(graph: &Vec<Vec<i32>>, start: i32, target_dist: i32) -> Vec<i32> {
    let mut distances = vec![i32::MAX; graph.len()];
    distances[start as usize] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = distances[node as usize];

        if dist > min_dist || dist > target_dist {
            continue;
        }

        for &neighbor in graph[node as usize].iter() {
            let neighbor_min_dist = distances[neighbor as usize];
            let new_dist = min_dist + 1;

            if new_dist >= neighbor_min_dist || new_dist > target_dist {
                continue;
            }

            distances[neighbor as usize] = new_dist;
            queue.push(Reverse((new_dist, neighbor)));
        }
    }

    distances
}
