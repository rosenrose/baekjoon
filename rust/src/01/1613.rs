use std::fmt::Write;
use std::io;

const MAX: usize = 400;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input(), input());
    let mut adjacency_matrix = [[false; MAX]; MAX];

    for (a, b) in (0..m).map(|_| (input() - 1, input() - 1)) {
        adjacency_matrix[a][b] = true;
    }

    floyd_warshall(&mut adjacency_matrix, n);

    for (a, b) in (0..input()).map(|_| (input() - 1, input() - 1)) {
        writeln!(
            output,
            "{}",
            match (adjacency_matrix[a][b], adjacency_matrix[b][a]) {
                (true, false) => -1,
                (false, true) => 1,
                (false, false) => 0,
                _ => unreachable!(),
            }
        )
        .unwrap();
    }

    print!("{output}");
}

fn floyd_warshall(graph: &mut [[bool; MAX]], graph_len: usize) {
    for stopby in 0..graph_len {
        for start in 0..graph_len {
            for end in 0..graph_len {
                graph[start][end] =
                    graph[start][end] || (graph[start][stopby] && graph[stopby][end]);
            }
        }
    }
}
