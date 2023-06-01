use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for i in 1.. {
        let (n, m) = (input(), input());

        if (n, m) == (0, 0) {
            return;
        }

        let mut adjacency_list = vec![Vec::new(); n as usize + 1];

        for (a, b) in (0..m).map(|_| (input(), input())) {
            adjacency_list[a as usize].push(b);
            adjacency_list[b as usize].push(a);
        }

        let count = dfs(&adjacency_list);

        print!("Case {i}: ");

        match count {
            0 => println!("No trees."),
            1 => println!("There is one tree."),
            _ => println!("A forest of {count} trees."),
        }
    }
}

fn dfs(graph: &[Vec<i32>]) -> i32 {
    let mut count = 0;
    let mut visited = vec![None; graph.len()];

    for start in 1..graph.len() {
        if visited[start].is_some() {
            continue;
        }

        visited[start] = Some(start as i32);
        let mut is_tree = true;
        let mut stack = vec![start as i32];

        while let Some(node) = stack.pop() {
            let parent = visited[node as usize].unwrap();

            for &adj in &graph[node as usize] {
                if adj == node {
                    is_tree = false;
                    continue;
                }

                if visited[adj as usize].is_some() {
                    if adj != parent {
                        is_tree = false;
                    }

                    continue;
                };

                visited[adj as usize] = Some(node);
                stack.push(adj);
            }
        }

        if is_tree {
            count += 1;
        }
    }

    count
}
