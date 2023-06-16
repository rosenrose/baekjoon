use std::fmt::Write;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = io::read_to_string(io::stdin())?;
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input(), input());
    let mut adjacency_matrix: Vec<Vec<_>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 0 } else { i32::MAX }).collect())
        .collect();

    for (a, b, c) in (0..m).map(|_| (input() - 1, input() - 1, input() as i32)) {
        adjacency_matrix[a][b] = c;
        adjacency_matrix[b][a] = c;
    }

    let prevs = floyd_warshall_with_path(&mut adjacency_matrix);

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

fn floyd_warshall_with_path(graph: &mut Vec<Vec<i32>>) -> Vec<Vec<Option<u8>>> {
    let len = graph.len();
    let mut prevs: Vec<Vec<_>> = graph
        .iter()
        .enumerate()
        .map(|(start, row)| {
            row.iter()
                .map(|&dist| (dist != i32::MAX).then(|| start as u8))
                .collect()
        })
        .collect();

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

fn get_second(prevs: &[Vec<Option<u8>>], start: usize, end: usize) -> u8 {
    let mut node = prevs[start][end].unwrap() as usize;
    let mut second = None;

    while node != start {
        second = Some(node);
        node = prevs[start][node].unwrap() as usize;
    }

    second.unwrap_or(end) as u8
}
