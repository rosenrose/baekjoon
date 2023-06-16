use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); n as usize + 1];

    for (a, b) in (0..m).map(|_| (input(), input())) {
        adjacency_list[a as usize].push(b);
    }

    let mut tall_counts = vec![0; adjacency_list.len()];
    let mut short_counts = vec![0; adjacency_list.len()];

    for start in 1..=n {
        let mut visited = vec![false; adjacency_list.len()];
        visited[start as usize] = true;

        let mut stack = vec![start];

        while let Some(node) = stack.pop() {
            for &adj in &adjacency_list[node as usize] {
                if visited[adj as usize] {
                    continue;
                }

                visited[adj as usize] = true;
                tall_counts[start as usize] += 1;
                short_counts[adj as usize] += 1;
                stack.push(adj);
            }
        }
    }

    let count = tall_counts[1..]
        .iter()
        .zip(&short_counts[1..])
        .filter(|&(tall, short)| tall + short == n - 1)
        .count();

    println!("{count}");
}
