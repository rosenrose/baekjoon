use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for i in 1..=input() {
        let (n, m) = (input(), input());
        let mut adjacency_list = vec![Vec::new(); m];

        for [x, y, z] in (0..n).map(|_| [(); 3].map(|_| input())) {
            adjacency_list[x].push((y, z as i32));
            adjacency_list[y].push((x, z as i32));
        }

        let (start, mut end) = (0, m - 1);
        let (distance, prevs) = dijkstra_with_path(&adjacency_list, start, end);

        write!(output, "Case #{i}: ")?;

        if distance == i32::MAX {
            writeln!(output, "-1")?;
            continue;
        }

        let mut path = vec![end];

        while end != start {
            end = prevs[end];
            path.push(end);
        }

        for p in path.iter().rev() {
            write!(output, "{p} ")?;
        }
        writeln!(output, "")?;
    }

    print!("{output}");
    Ok(())
}

fn dijkstra_with_path(graph: &[Vec<(usize, i32)>], start: usize, end: usize) -> (i32, Vec<usize>) {
    let mut distances = vec![i32::MAX; graph.len()];
    distances[start] = 0;

    let mut prevs = vec![0; graph.len()];
    let mut queue = BinaryHeap::from([(Reverse(0), start)]);

    while let Some((Reverse(dist), node)) = queue.pop() {
        let min_dist = distances[node];

        if node == end {
            return (min_dist, prevs);
        }
        if dist > min_dist {
            continue;
        }

        for &(adj, weight) in &graph[node] {
            let adj_min_dist = distances[adj];
            let new_dist = min_dist + weight;

            if new_dist >= adj_min_dist {
                continue;
            }

            distances[adj] = new_dist;
            prevs[adj] = node;

            queue.push((Reverse(new_dist), adj));
        }
    }

    (distances[end], prevs)
}
