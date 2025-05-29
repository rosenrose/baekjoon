use std::io;

const MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut adjacency_matrix = [[0; MAX]; MAX];

    for r in 0..n {
        for c in 0..n {
            adjacency_matrix[r][c] = if r == c { 0 } else { i32::MAX };
        }
    }

    for (r, row) in input.enumerate() {
        for (c, ch) in row.char_indices() {
            if ch == 'Y' {
                adjacency_matrix[r][c] = 1;
            }
        }
    }

    floyd_warshall(&mut adjacency_matrix[..n]);

    let count = adjacency_matrix[..n]
        .iter()
        .map(|row| {
            row[..n]
                .iter()
                .filter(|&&dist| 0 < dist && dist <= 2)
                .count()
        })
        .max()
        .unwrap();

    println!("{count}");
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
