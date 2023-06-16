use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut adjacency_matrix = vec![vec![i32::MAX; n]; n];

    for (a, b, c) in (0..m).map(|_| (input() - 1, input() - 1, input() as i32)) {
        adjacency_matrix[a][b] = c;
    }

    floyd_warshall(&mut adjacency_matrix);

    let min_dist = (0..n)
        .filter_map(|i| {
            let cycle_len = adjacency_matrix[i][i];
            (cycle_len != i32::MAX).then_some(cycle_len)
        })
        .min()
        .unwrap_or(-1);

    println!("{min_dist}");
}

fn floyd_warshall(graph: &mut Vec<Vec<i32>>) {
    let len = graph.len();

    for k in 0..len {
        for i in 0..len {
            for j in 0..len {
                graph[i][j] = graph[i][j].min(graph[i][k].saturating_add(graph[k][j]));
            }
        }
    }
}
