use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let n = input.next().unwrap();
    let mut adjacency_matrix: Vec<Vec<_>> = (0..n)
        .map(|_| input.by_ref().take(n).map(|num| num == 1).collect())
        .collect();

    floyd_warshall(&mut adjacency_matrix);

    for row in adjacency_matrix {
        for is_connected in row {
            write!(output, "{} ", is_connected as u8).unwrap();
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
}

fn floyd_warshall(graph: &mut Vec<Vec<bool>>) {
    let len = graph.len();

    for stopby in 0..len {
        for start in 0..len {
            for end in 0..len {
                graph[start][end] =
                    graph[start][end] || (graph[start][stopby] && graph[stopby][end]);
            }
        }
    }
}
