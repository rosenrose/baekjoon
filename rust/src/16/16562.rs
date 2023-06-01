use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m, mut k) = (input(), input(), input());
    let mut costs = vec![0; n + 1];

    for i in 1..=n {
        costs[i] = input();
    }

    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (v, w) in (0..m).map(|_| (input(), input())) {
        adjacency_list[v].push(w);
        adjacency_list[w].push(v);
    }

    let mut cost = 0;
    let mut visited = vec![false; adjacency_list.len()];

    for start in 1..=n {
        if visited[start] {
            continue;
        }

        visited[start] = true;
        let mut min_cost = usize::MAX;
        let mut stack = vec![start];

        while let Some(node) = stack.pop() {
            min_cost = costs[node].min(min_cost);

            for &adj in &adjacency_list[node] {
                if visited[adj] {
                    continue;
                }

                visited[adj] = true;
                stack.push(adj);
            }
        }

        if k < min_cost {
            println!("Oh no");
            return;
        }

        k -= min_cost;
        cost += min_cost;
    }

    println!("{cost}");
}
