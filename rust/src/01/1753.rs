use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (v, e, k) = (input() as usize, input() as usize, input() as usize);
    let mut adjacency_array = (vec![i32::MAX; v + 1], vec![((0, 0), 0); e]);

    for (i, (u, v, w)) in (0..e).map(|i| (i, (input() as usize, input(), input()))) {
        let prev = adjacency_array.0[u];

        adjacency_array.0[u] = i as i32;
        adjacency_array.1[i] = ((v, w), prev);
    }

    let distances = dijkstra(&adjacency_array, k);
    // println!("{distances:?}");
    for &dist in &distances[1..] {
        if dist == i32::MAX {
            writeln!(output, "INF")
        } else {
            writeln!(output, "{dist}")
        }
        .unwrap();
    }

    print!("{output}");
}

fn dijkstra((nodes, edges): &(Vec<i32>, Vec<((i32, i32), i32)>), start: usize) -> Vec<i32> {
    let mut distances = vec![i32::MAX; nodes.len()];
    distances[start] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start as i32))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = distances[node as usize];
        let mut edge = nodes[node as usize];

        if dist > min_dist || edge == i32::MAX {
            continue;
        }

        loop {
            let ((adj, weight), next_edge) = edges[edge as usize];
            let adj_min_dist = distances[adj as usize];
            let new_dist = min_dist + weight;

            if new_dist < adj_min_dist {
                distances[adj as usize] = new_dist;
                queue.push(Reverse((new_dist, adj)));
            }

            if next_edge == i32::MAX {
                break;
            }

            edge = next_edge;
        }
    }

    distances
}

// let mut adjacency_array = (vec![None; v + 1], vec![((0, 0), None); e]);

// for (i, (u, v, w)) in (0..e).map(|i| (i, (input(), input(), input() as i32))) {
//     let prev = adjacency_array.0[u];

//     adjacency_array.0[u] = Some(i);
//     adjacency_array.1[i] = ((v, w), prev);
// }

// fn dijkstra(
//     (nodes, edges): &(Vec<Option<usize>>, Vec<((usize, i32), Option<usize>)>),
//     start: usize,
// ) -> Vec<i32> {
//     let mut distances = vec![i32::MAX; nodes.len()];
//     distances[start] = 0;

//     let mut queue = BinaryHeap::from([Reverse((0, start))]);

//     while let Some(Reverse((dist, node))) = queue.pop() {
//         let min_dist = distances[node];

//         if dist > min_dist {
//             continue;
//         }
//         let Some(mut edge) = nodes[node] else {
//             continue;
//         };

//         loop {
//             let (adj, weight) = edges[edge].0;
//             let adj_min_dist = distances[adj];
//             let new_dist = min_dist + weight;

//             if new_dist < adj_min_dist {
//                 distances[adj] = new_dist;
//                 queue.push(Reverse((new_dist, adj)));
//             }

//             let Some(next_edge) = edges[edge].1 else {
//                 break;
//             };

//             edge = next_edge;
//         }
//     }

//     distances
// }
