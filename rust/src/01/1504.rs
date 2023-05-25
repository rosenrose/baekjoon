use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, e) = (input() as usize, input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (a, b, c) in (0..e).map(|_| (input(), input(), input())) {
        adjacency_list[a as usize].push((b, c));
        adjacency_list[b as usize].push((a, c));
    }

    let (v1, v2) = (input() as usize, input() as usize);

    let dists_from_1 = dijkstra(&adjacency_list, 1);
    let dists_from_v1 = dijkstra(&adjacency_list, v1);
    let dists_from_v2 = dijkstra(&adjacency_list, v2);
    // println!("{dists1:?} {dists2:?} {dists3:?}");
    let result1 = [dists_from_1[v1], dists_from_v1[v2], dists_from_v2[n]];
    let result2 = [dists_from_1[v2], dists_from_v2[v1], dists_from_v1[n]];

    if result1.contains(&i32::MAX) && result2.contains(&i32::MAX) {
        println!("-1");
        return;
    }

    println!(
        "{}",
        result1.iter().sum::<i32>().min(result2.iter().sum::<i32>())
    );
}

fn dijkstra(graph: &[Vec<(i32, i32)>], start: usize) -> Vec<i32> {
    let mut dists = vec![i32::MAX; graph.len()];
    dists[start] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start as i32))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = dists[node as usize];

        if dist > min_dist {
            continue;
        }

        for &(adj, weight) in graph[node as usize].iter() {
            let adj_min_dist = dists[adj as usize];
            let new_dist = min_dist + weight;

            if new_dist >= adj_min_dist {
                continue;
            }

            dists[adj as usize] = new_dist;
            queue.push(Reverse((new_dist, adj)));
        }
    }

    dists
}
