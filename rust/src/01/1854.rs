use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, m, k) = (input(), input(), input());

    let adjacency_list = (0..m).fold(vec![Vec::new(); n + 1], |mut acc, _| {
        let (a, b, c) = (input(), input(), input() as i32);
        acc[a].push((b, c));

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

    while let Some(Reverse((dist, node))) = queue.pop() {
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
