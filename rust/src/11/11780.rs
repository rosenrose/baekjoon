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

    let prevs = floyd_warshall_with_path(&mut adjacency_matrix);

    for row in adjacency_matrix {
        for dist in row {
            write!(output, "{} ", if dist == i32::MAX { 0 } else { dist }).unwrap();
        }
        writeln!(output, "").unwrap();
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                writeln!(output, "0").unwrap();
                continue;
            }

            let Some(path) = find_path(&prevs, i, j) else {
                writeln!(output, "0").unwrap();
                continue;
            };

            write!(output, "{} ", path.len() + 1).unwrap();

            for p in path {
                write!(output, "{} ", p + 1).unwrap();
            }

            writeln!(output, "{}", j + 1).unwrap();
        }
    }

    print!("{output}");
}

fn floyd_warshall_with_path(distances: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = distances.len();
    let mut path: Vec<Vec<_>> = distances
        .iter()
        .enumerate()
        .map(|(from, row)| {
            row.iter()
                .map(|&d| if d == i32::MAX { -1 } else { from as i32 })
                .collect()
        })
        .collect();

    for k in 0..len {
        for i in 0..len {
            for j in 0..len {
                let new_dist = distances[i][k].saturating_add(distances[k][j]);

                if new_dist < distances[i][j] {
                    distances[i][j] = new_dist;
                    path[i][j] = k as i32;
                }
            }
        }
    }

    path
}

fn find_path(prevs: &[Vec<i32>], start: usize, end: usize) -> Option<Vec<usize>> {
    let prev = prevs[start][end];

    if prev == -1 {
        return None;
    }

    let prev = prev as usize;

    if prev == start {
        return Some(vec![start]);
    }

    let start_prev = find_path(prevs, start, prev)?;
    let prev_end = find_path(prevs, prev, end)?;

    Some([start_prev, prev_end].concat())
}
