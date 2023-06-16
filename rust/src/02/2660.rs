use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut adjacency_matrix: Vec<Vec<_>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 0 } else { i32::MAX }).collect())
        .collect();

    while let (a @ 0.., b @ 0..) = (input() - 1, input() - 1) {
        adjacency_matrix[a as usize][b as usize] = 1;
        adjacency_matrix[b as usize][a as usize] = 1;
    }

    floyd_warshall(&mut adjacency_matrix);

    let mut min_score = i32::MAX;
    let scores: Vec<_> = adjacency_matrix
        .iter()
        .map(|row| {
            let score = *row.iter().max().unwrap();
            min_score = score.min(min_score);

            score
        })
        .collect();

    let chiefs: Vec<_> = scores
        .iter()
        .enumerate()
        .filter_map(|(num, &score)| (score == min_score).then_some(num + 1))
        .collect();

    println!("{min_score} {}", chiefs.len());

    for chief in chiefs {
        print!("{chief} ");
    }
}

fn floyd_warshall(graph: &mut Vec<Vec<i32>>) {
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
