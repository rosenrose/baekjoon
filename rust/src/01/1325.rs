use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (a, b) in (0..m).map(|_| (input(), input())) {
        adjacency_list[b as usize].push(a);
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

fn dfs(graph: &[Vec<i32>]) -> (Vec<i32>, i32) {
    let mut counts = vec![0; graph.len()];
    let mut max_count = 0;

    for start in 1..graph.len() {
        let mut visited = vec![false; graph.len()];
        let mut count = 0;
        let mut stack = vec![start as i32];
        visited[start] = true;

        while let Some(node) = stack.pop() {
            count += 1;

            for &adj in graph[node as usize].iter() {
                if visited[adj as usize] {
                    continue;
                }

                visited[adj as usize] = true;
                stack.push(adj);
            }
        }

        counts[start] = count;
        max_count = count.max(max_count);
    }

    (counts, max_count)
}
