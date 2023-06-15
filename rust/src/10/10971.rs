use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let adjacency_matrix: Vec<Vec<_>> = (0..n)
        .map(|_| {
            input
                .by_ref()
                .take(n)
                .map(|num| if num == 0 { i32::MAX } else { num })
                .collect()
        })
        .collect();

    let mut min_cost = i32::MAX;

    permutations(
        0,
        &mut vec![0; n],
        &mut [false; 10],
        &adjacency_matrix,
        0,
        &mut min_cost,
    );

    println!("{min_cost}");
}

fn permutations(
    depth: usize,
    selected: &mut Vec<usize>,
    visited: &mut [bool],
    graph: &[Vec<i32>],
    cost: i32,
    min_cost: &mut i32,
) {
    if depth == selected.len() {
        *min_cost = cost
            .saturating_add(graph[*selected.last().unwrap()][selected[0]])
            .min(*min_cost);
        return;
    }

    for i in 0..selected.len() {
        if visited[i] {
            continue;
        }

        selected[depth] = i;

        let new_cost = cost.saturating_add(if depth > 0 {
            graph[selected[depth - 1]][selected[depth]]
        } else {
            0
        });

        if new_cost >= *min_cost {
            continue;
        }

        visited[i] = true;

        permutations(depth + 1, selected, visited, graph, new_cost, min_cost);

        visited[i] = false;
    }
}
