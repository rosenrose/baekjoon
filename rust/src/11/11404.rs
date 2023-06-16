use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input(), input());
    let mut adjacency_matrix: Vec<Vec<_>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 0 } else { i32::MAX }).collect())
        .collect();

    for (a, b, c) in (0..m).map(|_| (input() - 1, input() - 1, input() as i32)) {
        adjacency_matrix[a][b] = adjacency_matrix[a][b].min(c);
    }

    floyd_warshall(&mut adjacency_matrix);

    for row in adjacency_matrix {
        for dist in row {
            write!(output, "{} ", if dist == i32::MAX { 0 } else { dist }).unwrap();
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
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
