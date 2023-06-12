use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for [a, b, w] in (0..n - 1).map(|_| [(); 3].map(|_| input())) {
        adjacency_list[a].push((b, w as i32));
        adjacency_list[b].push((a, w as i32));
    }

    'outer: for (start, end) in (0..m).map(|_| (input(), input())) {
        let mut visited = vec![false; adjacency_list.len()];
        visited[start] = true;

        let mut stack = vec![(start, 0)];

        while let Some((node, dist)) = stack.pop() {
            for &(adj, weight) in &adjacency_list[node] {
                let new_dist = dist + weight;

                if adj == end {
                    writeln!(output, "{new_dist}").unwrap();
                    continue 'outer;
                }

                if visited[adj] {
                    continue;
                }

                visited[adj] = true;
                stack.push((adj, new_dist));
            }
        }
    }

    print!("{output}");
}
