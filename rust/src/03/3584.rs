use std::io;

const MAX: usize = 10000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let n = input();
        let mut adjacency_list = [(); MAX].map(|_| Vec::new());
        let mut parents = [0; MAX];

        for (a, b) in (0..n - 1).map(|_| (input(), input())) {
            adjacency_list[a].push(b);
            parents[b] = a;
        }

        let root = parents[1..=n].iter().position(|&p| p == 0).unwrap() + 1;
        let mut depths = [0; MAX];
        let mut stack = vec![(root, 1)];

        while let Some((node, depth)) = stack.pop() {
            depths[node] = depth;

            for &adj in &adjacency_list[node] {
                stack.push((adj, depth + 1));
            }
        }

        println!("{}", lca((input(), input()), &depths[..=n], &parents[..=n]));
    }
}

fn lca((mut u, mut v): (usize, usize), depths: &[i32], parents: &[usize]) -> usize {
    while depths[u] > depths[v] {
        u = parents[u];
    }
    while depths[v] > depths[u] {
        v = parents[v];
    }

    while u != v {
        u = parents[u];
        v = parents[v];
    }

    u
}
