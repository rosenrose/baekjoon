use std::fmt::Write;
use std::io;

const MAX: usize = 10000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input());
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for (a, b) in (0..m).map(|_| (input(), input())) {
        adjacency_list[b as usize].push(a as i16);
    }

    let (counts, max_count) = dfs(&adjacency_list, n + 1);

    for node in counts[..=n]
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(node, &count)| (count == max_count).then_some(node))
    {
        write!(output, "{node} ").unwrap();
    }

    print!("{output}");
}

fn dfs(graph: &[Vec<i16>], graph_len: usize) -> ([i16; MAX], i16) {
    let mut counts = [0; MAX];
    let mut max_count = 0;

    for start in 1..graph_len {
        let mut visited = [false; MAX];
        visited[start] = true;

        let mut count = 0;
        let mut stack = vec![start as i16];

        while let Some(node) = stack.pop() {
            count += 1;

            for &adj in &graph[node as usize] {
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
