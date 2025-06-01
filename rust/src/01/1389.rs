use std::io;

const MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut adjacency_matrix = [[0; MAX]; MAX];

    for r in 0..n {
        for c in 0..n {
            adjacency_matrix[r][c] = if r == c { 0 } else { i32::MAX };
        }
    }

    for [a, b] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap() - 1)) {
        adjacency_matrix[a][b] = 1;
        adjacency_matrix[b][a] = 1;
    }

    floyd_warshall(&mut adjacency_matrix[..n]);

    let (least_kevin_bacon, _) = adjacency_matrix[..n]
        .iter()
        .enumerate()
        .min_by_key(|(_, row)| row[..n].iter().sum::<i32>())
        .unwrap();

    println!("{}", least_kevin_bacon + 1);
}

fn floyd_warshall(graph: &mut [[i32; MAX]]) {
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
