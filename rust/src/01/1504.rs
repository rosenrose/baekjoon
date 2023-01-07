use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, e) = (input(), input());

    let adjacency_list = (0..e).fold(vec![Vec::new(); n + 1], |mut acc, _| {
        let (a, b, c) = (input(), input(), input() as i32);
        acc[a].push((b, c));
        acc[b].push((a, c));

        acc
    });

    let (v1, v2) = (input(), input());

    let dists_from_1 = dijkstra(&adjacency_list, 1);
    let dists_from_v1 = dijkstra(&adjacency_list, v1);
    let dists_from_v2 = dijkstra(&adjacency_list, v2);
    // println!("{dists1:?} {dists2:?} {dists3:?}");
    let result1 = [dists_from_1[v1], dists_from_v1[v2], dists_from_v2[n]];
    let result2 = [dists_from_1[v2], dists_from_v2[v1], dists_from_v1[n]];

    if result1.iter().any(|&d| d == i32::MAX) && result2.iter().any(|&d| d == i32::MAX) {
        println!("-1");
        return;
    }

    println!(
        "{}",
        result1.iter().sum::<i32>().min(result2.iter().sum::<i32>())
    );
}

fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> Vec<i32> {
    let mut dists = vec![i32::MAX; graph.len()];
    dists[start] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = dists[node];

        if dist > min_dist {
            continue;
        }

        for &(neighbor, weight) in graph[node].iter() {
            let neighbor_min_dist = dists[neighbor];
            let new_dist = min_dist + weight;

            if new_dist >= neighbor_min_dist {
                continue;
            }

            dists[neighbor] = new_dist;
            queue.push(Reverse((new_dist, neighbor)));
        }
    }

    dists
}
