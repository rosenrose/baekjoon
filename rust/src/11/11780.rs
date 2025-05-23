use core::iter::Iterator;
use std::fmt::Write;
use std::io;

const MAX: usize = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = io::read_to_string(io::stdin())?;
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut adjacency_matrix = [[0; MAX]; MAX];

    for r in 0..n {
        for c in 0..n {
            adjacency_matrix[r][c] = if r == c { 0 } else { i32::MAX };
        }
    }

    for [a, b, c] in (0..m).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        adjacency_matrix[a - 1][b - 1] = adjacency_matrix[a - 1][b - 1].min(c as i32);
    }

    let prevs = floyd_warshall_with_path(&mut adjacency_matrix[..n]);

    for row in &adjacency_matrix[..n] {
        for &dist in &row[..n] {
            write!(output, "{} ", if dist == i32::MAX { 0 } else { dist })?;
        }
        writeln!(output, "")?;
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                writeln!(output, "0")?;
                continue;
            }
            let Some((path, path_len)) = get_path(&prevs[..n], i, j) else {
                writeln!(output, "0")?;
                continue;
            };

            write!(output, "{path_len} ")?;

            for p in path[..path_len].iter().rev() {
                write!(output, "{} ", p + 1)?;
            }

            writeln!(output, "")?;
        }
    }

    print!("{output}");
    Ok(())
}

fn floyd_warshall_with_path(graph: &mut [[i32; MAX]]) -> [[Option<u8>; MAX]; MAX] {
    let len = graph.len();
    let mut prevs = [[None; MAX]; MAX];

    for (start, row) in graph.iter().enumerate() {
        for (c, &dist) in row.iter().enumerate() {
            prevs[start][c] = (dist != i32::MAX).then(|| start as u8);
        }
    }

    for stopby in 0..len {
        for start in 0..len {
            for end in 0..len {
                let new_dist = graph[start][stopby].saturating_add(graph[stopby][end]);

                if new_dist < graph[start][end] {
                    graph[start][end] = new_dist;
                    prevs[start][end] = prevs[stopby][end];
                }
            }
        }
    }

    prevs
}

fn get_path(prevs: &[[Option<u8>; MAX]], start: usize, end: usize) -> Option<([u8; MAX], usize)> {
    let mut node = prevs[start][end]? as usize;
    let mut path = [0; MAX];
    path[0] = end as u8;
    let mut path_len = 1;

    while node != start {
        path[path_len] = node as u8;
        path_len += 1;
        node = prevs[start][node]? as usize;
    }

    path[path_len] = start as u8;
    path_len += 1;

    Some((path, path_len))
}
