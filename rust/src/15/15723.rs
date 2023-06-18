use std::io;

const SIZE: usize = 26;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n = parse_int(input());
    let mut adjacency_matrix = [[false; SIZE]; SIZE];

    for [a, _, b] in (0..n).map(|_| [(); 3].map(|_| (input().as_bytes()[0] - b'a') as usize)) {
        adjacency_matrix[a][b] = true;
    }

    floyd_warshall(&mut adjacency_matrix);

    let m = parse_int(input());

    for [a, _, b] in (0..m).map(|_| [(); 3].map(|_| (input().as_bytes()[0] - b'a') as usize)) {
        println!("{}", if adjacency_matrix[a][b] { 'T' } else { 'F' });
    }
}

fn floyd_warshall(graph: &mut [[bool; SIZE]]) {
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

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
