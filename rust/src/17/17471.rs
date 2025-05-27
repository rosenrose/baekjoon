use std::io;

const MAX: usize = 10 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut populations = [0; MAX];

    for (i, num) in input.by_ref().take(n).enumerate() {
        populations[i + 1] = num as i32;
    }

    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for i in 1..=n {
        let adj_count = input.next().unwrap();
        adjacency_list[i] = input.by_ref().take(adj_count).collect();
    }

    let min_diff = (1..=n / 2)
        .flat_map(|i| {
            combinations(
                0,
                1,
                &mut [0; MAX / 2][..i],
                &adjacency_list[..=n],
                &populations[..=n],
            )
        })
        .min()
        .unwrap_or(-1);

    println!("{min_diff}")
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut [usize],
    graph: &[Vec<usize>],
    populations: &[i32],
) -> Option<i32> {
    if depth == selected.len() {
        let mut rest = [0; MAX];
        let mut rest_len = 0;

        for i in (1..graph.len()).filter(|n| !selected.contains(n)) {
            rest[rest_len] = i;
            rest_len += 1;
        }

        return get_population_diff(selected, &rest[..rest_len], graph, populations);
    }

    let takes = graph.len() - (selected.len() - 1);

    (start..depth + takes)
        .flat_map(|i| {
            selected[depth] = i;
            combinations(depth + 1, i + 1, selected, graph, populations)
        })
        .min()
}

fn get_population_diff(
    district_a: &[usize],
    district_b: &[usize],
    graph: &[Vec<usize>],
    populations: &[i32],
) -> Option<i32> {
    let (mut population_a, mut population_b) = (0, 0);
    let mut visited = [false; MAX];
    visited[district_a[0]] = true;

    let mut stack = vec![district_a[0]];

    while let Some(node) = stack.pop() {
        population_a += populations[node];

        for &adj in &graph[node] {
            if visited[adj] || district_b.contains(&adj) {
                continue;
            }

            visited[adj] = true;
            stack.push(adj);
        }
    }

    if !district_a.iter().all(|&node| visited[node]) {
        return None;
    }

    visited[district_b[0]] = true;
    stack.push(district_b[0]);

    while let Some(node) = stack.pop() {
        population_b += populations[node];

        for &adj in &graph[node] {
            if visited[adj] || district_a.contains(&adj) {
                continue;
            }

            visited[adj] = true;
            stack.push(adj);
        }
    }

    district_b
        .iter()
        .all(|&node| visited[node])
        .then(|| population_a.abs_diff(population_b) as i32)
}
