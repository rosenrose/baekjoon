use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input(), input());
    let mut adjacency_matrix: Vec<Vec<_>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 0 } else { i32::MAX }).collect())
        .collect();

    for _ in 0..m {
        let (a, b, c) = (input() - 1, input() - 1, input() as i32);
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

fn floyd_warshall(distances: &mut Vec<Vec<i32>>) {
    let len = distances.len();

    for k in 0..len {
        for i in 0..len {
            for j in 0..len {
                distances[i][j] =
                    distances[i][j].min(distances[i][k].saturating_add(distances[k][j]));
            }
        }
    }
}
