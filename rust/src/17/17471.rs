use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut populations = vec![0; n + 1];

    for (i, num) in input.by_ref().take(n).enumerate() {
        populations[i + 1] = num as i32;
    }

    let mut adjacency_list = vec![Vec::new(); n + 1];

    for i in 1..=n {
        let adj_count = input.next().unwrap();
        adjacency_list[i].extend(input.by_ref().take(adj_count));
    }

    let min_diff = (1..=n / 2)
        .flat_map(|i| combinations(0, 1, &mut vec![0; i], &adjacency_list, &populations))
        .min()
        .unwrap_or(-1);

    println!("{min_diff}")
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
    graph: &[Vec<usize>],
    populations: &[i32],
) -> Option<i32> {
    if depth == selected.len() {
        let rest: Vec<_> = (1..graph.len()).filter(|n| !selected.contains(n)).collect();

        return get_population_diff(selected, &rest, graph, populations);
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
    let mut visited = vec![false; graph.len()];
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
