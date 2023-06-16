use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut adjacency_matrix = vec![vec![false; n]; n];

    for (a, b) in (0..m).map(|_| (input() - 1, input() - 1)) {
        adjacency_matrix[a][b] = true;
    }

    floyd_warshall(&mut adjacency_matrix);

    for i in 0..n {
        let comparables = (0..n)
            .filter(|&j| {
                if i == j {
                    return true;
                }

                adjacency_matrix[i][j] || adjacency_matrix[j][i]
            })
            .count();

        println!("{}", n - comparables);
    }
}

fn floyd_warshall(graph: &mut Vec<Vec<bool>>) {
    let len = graph.len();

    for k in 0..len {
        for i in 0..len {
            for j in 0..len {
                graph[i][j] = graph[i][j] || (graph[i][k] && graph[k][j]);
            }
        }
    }
}
