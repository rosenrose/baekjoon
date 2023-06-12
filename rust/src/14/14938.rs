use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let [n, range, m] = [(); 3].map(|_| input());
    let items: Vec<_> = (0..n).map(|_| input()).collect();

    let mut adjacency_matrix: Vec<Vec<_>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 0 } else { i32::MAX }).collect())
        .collect();

    for (a, b, len) in (0..m).map(|_| (input() as usize - 1, input() as usize - 1, input())) {
        adjacency_matrix[a][b] = len;
        adjacency_matrix[b][a] = len;
    }

    floyd_warshall(&mut adjacency_matrix);

    let max_count: i32 = adjacency_matrix
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter_map(|(i, &dist)| (dist <= range).then_some(items[i]))
                .sum()
        })
        .max()
        .unwrap();

    println!("{max_count}");
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
