use std::io;

const MAX: usize = 10000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m, mut k] = [(); 3].map(|_| input.next().unwrap());
    let mut costs = [0; MAX];

    for (i, num) in input.by_ref().take(n).enumerate() {
        costs[i + 1] = num;
    }

    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for [v, w] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        adjacency_list[v].push(w);
        adjacency_list[w].push(v);
    }

    let mut cost = 0;
    let mut visited = [false; MAX];

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
