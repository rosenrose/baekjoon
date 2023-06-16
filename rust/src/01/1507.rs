use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let adjacency_matrix: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();
    let Some(removed) = floyd_warshall(&adjacency_matrix) else {
        println!("-1");
        return;
    };

    let sum: i32 = (0..n)
        .map(|i| {
            (i + 1..n)
                .filter_map(|j| (!removed[i][j]).then_some(adjacency_matrix[i][j]))
                .sum::<i32>()
        })
        .sum();

    println!("{sum}");
}

fn floyd_warshall(graph: &Vec<Vec<i32>>) -> Option<Vec<Vec<bool>>> {
    let len = graph.len();
    let mut removed = vec![vec![false; len]; len];

    for stopby in 0..len {
        for start in 0..len {
            for end in 0..len {
                if start == end || start == stopby || stopby == end {
                    continue;
                }

                let min_dist = graph[start][end];
                let new_dist = graph[start][stopby] + graph[stopby][end];

                if new_dist < min_dist {
                    return None;
                }
                if new_dist == min_dist {
                    removed[start][end] = true;
                };
            }
        }
    }

    Some(removed)
}
