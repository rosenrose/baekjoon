use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut adjacency_array = (vec![i32::MAX; n + 1], vec![(0, 0); m]);

    for (i, [u, v]) in (0..m).map(|i| (i, [(); 2].map(|_| input.next().unwrap()))) {
        let prev = adjacency_array.0[u];

        adjacency_array.0[u] = i as i32;
        adjacency_array.1[i] = (v as i32, prev);
    }

    let mut fans = vec![false; n + 1];

    for fan in input.skip(1) {
        fans[fan as usize] = true;
    }

    let is_encounter = dfs(&adjacency_array, 1, &fans);

    println!("{}", if is_encounter { "Yes" } else { "yes" });
}

fn dfs((nodes, edges): &(Vec<i32>, Vec<(i32, i32)>), start: usize, fans: &[bool]) -> bool {
    let mut stack = vec![start as i32];

    while let Some(node) = stack.pop() {
        if fans[node as usize] {
            continue;
        }

        let mut edge = nodes[node as usize];

        if edge == i32::MAX {
            return false;
        }

        while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
            stack.push(adj);
            edge = next_edge;
        }
    }

    true
}
