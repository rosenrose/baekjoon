use std::io;

const MAX: usize = 500 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for (a, b) in (0..m).map(|_| (input(), input())) {
        adjacency_list[a].push(b);
        adjacency_list[b].push(a);
    }

    let mut count = 0;
    let mut visited = [false; MAX];
    visited[1] = true;

    let mut stack = vec![(1, 0)];

    while let Some((node, dist)) = stack.pop() {
        if dist == 2 {
            continue;
        }

        for &adj in &adjacency_list[node] {
            if visited[adj] {
                continue;
            }

            visited[adj] = true;
            count += 1;
            stack.push((adj, dist + 1));
        }
    }

    println!("{count}");
}
