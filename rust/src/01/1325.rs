use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (a, b) in (0..m).map(|_| (input(), input())) {
        adjacency_list[b].push(a);
    }

    let (counts, max_count) = dfs(&adjacency_list);

    for node in counts
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(node, &count)| (count == max_count).then_some(node))
    {
        write!(output, "{node} ").unwrap();
    }

    print!("{output}");
}

fn dfs(graph: &[Vec<usize>]) -> (Vec<i32>, i32) {
    let mut counts = vec![0; graph.len()];
    let mut max_count = 0;

    for start in 1..graph.len() {
        let mut visited = vec![false; graph.len()];
        let mut count = 0;
        let mut stack = vec![start];

        while let Some(node) = stack.pop() {
            if visited[node] {
                continue;
            }

            visited[node] = true;
            count += 1;

            for &neighbor in graph[node].iter() {
                if visited[neighbor] {
                    continue;
                }

                stack.push(neighbor);
            }
        }

        counts[start] = count;
        max_count = count.max(max_count);
    }

    (counts, max_count)
}
