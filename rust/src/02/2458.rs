use std::io;

const MAX: usize = 500 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for [a, b] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        adjacency_list[a as usize].push(b);
    }

    let mut tall_counts = [0; MAX];
    let mut short_counts = [0; MAX];

    for start in 1..=n {
        let mut visited = [false; MAX];
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
                stack.push(adj as usize);
            }
        }
    }

    let count = tall_counts[1..=n]
        .iter()
        .zip(&short_counts[1..=n])
        .filter(|&(tall, short)| tall + short == n - 1)
        .count();

    println!("{count}");
}
