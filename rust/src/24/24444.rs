use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, m, r) = (input(), input(), input());

    let mut adjacency_list = (0..m).fold(vec![Vec::new(); n + 1], |mut acc, _| {
        let (u, v) = (input(), input());
        acc[u].push(v);
        acc[v].push(u);

        acc
    });

    for list in adjacency_list.iter_mut() {
        (*list).sort_unstable();
    }

    bfs(&adjacency_list, r);
}

fn bfs(graph: &Vec<Vec<usize>>, start: usize) {
    let mut visited = vec![0; graph.len()];
    let mut queue = VecDeque::from([start]);
    let mut count = 1;
    let mut output = String::new();

    while let Some(node) = queue.pop_front() {
        if visited[node] != 0 {
            continue;
        }

        visited[node] = count;
        count += 1;

        for &neighbor in graph[node].iter() {
            queue.push_back(neighbor);
        }
    }

    for order in visited.iter().skip(1) {
        writeln!(output, "{order}").unwrap();
    }

    print!("{output}");
}
