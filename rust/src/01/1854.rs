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

    let (n, m, k) = (input(), input(), input());

    let adjacency_list = (0..m).fold(vec![Vec::new(); n + 1], |mut acc, _| {
        let (a, b, c) = (input(), input(), input());
        acc[a].push((b, c as i32));

        acc
    });

    let distances = dijkstra_nth(&adjacency_list, 1, k);
    // println!("{distances:?}");
    for dists in distances.iter().skip(1) {
        println!(
            "{}",
            if dists.len() < k {
                -1
            } else {
                *dists.peek().unwrap()
            }
        );
    }
}

fn dijkstra_nth(
    graph: &Vec<Vec<(usize, i32)>>,
    start: usize,
    order: usize,
) -> Vec<BinaryHeap<i32>> {
    let mut distances = vec![BinaryHeap::new(); graph.len()];
    distances[start].push(0);

    let mut queue = BinaryHeap::from([Reverse((0, start))]);

    while !queue.is_empty() {
        let (dist, node) = queue.pop().unwrap().0;

        for &(neighbor, weight) in graph[node].iter() {
            let new_dist = dist + weight;

            if distances[neighbor].len() < order {
                distances[neighbor].push(new_dist);
                queue.push(Reverse((new_dist, neighbor)));

                continue;
            }

            if new_dist < *distances[neighbor].peek().unwrap() {
                distances[neighbor].pop();
                distances[neighbor].push(new_dist);
                queue.push(Reverse((new_dist, neighbor)));
            }
        }
    }

    distances
}
