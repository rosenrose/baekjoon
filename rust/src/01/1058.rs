use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: i32 = input.next().unwrap().parse().unwrap();
    let mut adjacency_matrix: Vec<Vec<_>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 0 } else { i32::MAX }).collect())
        .collect();

    for (r, row) in input.enumerate() {
        for (c, ch) in row.char_indices() {
            if ch == 'Y' {
                adjacency_matrix[r][c] = 1;
            }
        }
    }

    floyd_warshall(&mut adjacency_matrix);

    let count = adjacency_matrix
        .iter()
        .map(|row| row.iter().filter(|&&dist| 0 < dist && dist <= 2).count())
        .max()
        .unwrap();

    println!("{count}");
}

fn floyd_warshall(graph: &mut Vec<Vec<i32>>) {
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
