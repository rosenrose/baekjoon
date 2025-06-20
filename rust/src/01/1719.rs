use core::iter::Iterator;
use std::fmt::Write;
use std::io;

const MAX: usize = 200;

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
        adjacency_matrix[a - 1][b - 1] = c as i32;
        adjacency_matrix[b - 1][a - 1] = c as i32;
    }

    let prevs = floyd_warshall_with_path(&mut adjacency_matrix[..n]);

    for i in 0..n {
        for j in 0..n {
            if i == j {
                write!(output, "- ")?;
                continue;
            }

            write!(output, "{} ", get_second(&prevs, i, j) + 1)?;
        }
        writeln!(output, "")?;
    }

    print!("{output}");
    Ok(())
}

fn floyd_warshall_with_path(graph: &mut [[i32; MAX]]) -> [[Option<u8>; MAX]; MAX] {
    let len = graph.len();
    let mut prevs = [[None; MAX]; MAX];

    for (start, row) in graph.iter().enumerate() {
        for (c, &dist) in row[..len].iter().enumerate() {
            prevs[start][c] = (dist != i32::MAX).then_some(start as u8);
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

fn get_second(prevs: &[[Option<u8>; MAX]], start: usize, end: usize) -> u8 {
    let mut node = prevs[start][end].unwrap() as usize;
    let mut second = None;

    while node != start {
        second = Some(node);
        node = prevs[start][node].unwrap() as usize;
    }

    second.unwrap_or(end) as u8
}
