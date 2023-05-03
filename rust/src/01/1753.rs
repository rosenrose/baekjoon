use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (v, e, k) = (input(), input(), input());
    let mut adjacency_list = vec![Vec::new(); v + 1];

    for (u, v, w) in (0..e).map(|_| (input(), input(), input() as i32)) {
        adjacency_list[u].push((v, w));
    }

    let distances = dijkstra(&adjacency_list, k);
    // println!("{distances:?}");
    for &dist in &distances[1..] {
        (if dist == i32::MAX {
            writeln!(output, "INF")
        } else {
            writeln!(output, "{dist}")
        })
        .unwrap();
    }

    print!("{output}");
}

fn dijkstra(graph: &[Vec<(usize, i32)>], start: usize) -> Vec<i32> {
    let mut distances = vec![i32::MAX; graph.len()];
    distances[start] = 0;

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
            queue.push(Reverse((new_dist, neighbor)));
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
//             let (neighbor, weight) = edges[edge].0;
//             let neighbor_min_dist = distances[neighbor];
//             let new_dist = min_dist + weight;

//             if new_dist < neighbor_min_dist {
//                 distances[neighbor] = new_dist;
//                 queue.push(Reverse((new_dist, neighbor)));
//             }

//             let Some(next_edge) = edges[edge].1 else {
//                 break;
//             };

//             edge = next_edge;
//         }
//     }

//     distances
// }
