use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut adjacency_matrix = vec![vec![false; n]; n];

    for [a, b] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap() - 1)) {
        adjacency_matrix[a][b] = true;
    }

    floyd_warshall(&mut adjacency_matrix);

    let middle = (n + 1) / 2;
    let count: i32 = (0..n)
        .map(|i| {
            let is_heavy = adjacency_matrix[i].iter().filter(|&&b| b).count() >= middle;
            let is_light = adjacency_matrix.iter().filter(|row| row[i]).count() >= middle;

            (is_heavy || is_light) as i32
        })
        .sum();

    println!("{count}");
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
