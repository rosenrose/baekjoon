use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for i in 1..=input() {
        let (n, m) = (input(), input());
        let mut adjacency_list = vec![Vec::new(); m];

        for (x, y, z) in (0..n).map(|_| (input(), input(), input() as i32)) {
            adjacency_list[x].push((y, z));
            adjacency_list[y].push((x, z));
        }

        let (start, mut end) = (0, m - 1);
        let (distances, prevs) = dijkstra_with_path(&adjacency_list, start);

        write!(output, "Case #{i}: ").unwrap();

        if distances[end] == i32::MAX {
            writeln!(output, "-1").unwrap();
            continue;
        }

        let mut path = vec![end];

        while end != start {
            end = prevs[end];
            path.push(end);
        }

        for p in path.iter().rev() {
            write!(output, "{p} ").unwrap();
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
}

fn dijkstra_with_path(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> (Vec<i32>, Vec<usize>) {
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
