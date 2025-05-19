use std::io;

const MAX: usize = 20;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut adjacency_matrix = [[0; MAX]; MAX];

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            adjacency_matrix[r][c] = num;
        }
    }

    let Some(removed) = floyd_warshall(&adjacency_matrix, n) else {
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

fn floyd_warshall(graph: &[[i32; MAX]; MAX], graph_len: usize) -> Option<[[bool; MAX]; MAX]> {
    let mut removed = [[false; MAX]; MAX];

    for stopby in 0..graph_len {
        for start in 0..graph_len {
            for end in 0..graph_len {
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
