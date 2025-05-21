use std::io;

const MAX: usize = 400;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut adjacency_matrix = [[i32::MAX; MAX]; MAX];

    for [a, b, c] in (0..m).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        adjacency_matrix[a - 1][b - 1] = c as i32;
    }

    floyd_warshall(&mut adjacency_matrix[..n]);

    let min_dist = (0..n)
        .filter_map(|i| {
            let cycle_len = adjacency_matrix[i][i];
            (cycle_len != i32::MAX).then_some(cycle_len)
        })
        .min()
        .unwrap_or(-1);

    println!("{min_dist}");
}

fn floyd_warshall(graph: &mut [[i32; MAX]]) {
    let len = graph.len();

    for stopby in 0..len {
        for start in 0..len {
            for end in 0..len {
                graph[start][end] =
                    graph[start][end].min(graph[start][stopby].saturating_add(graph[stopby][end]));
            }
        }
    }
}
