use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut adjacency_array = (vec![i32::MAX; n + 1], Vec::with_capacity(m * 2));

    for [a, b] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        let prev = adjacency_array.0[a];
        adjacency_array.0[a] = adjacency_array.1.len() as i32;
        adjacency_array.1.push((b as i32, prev));

        let prev = adjacency_array.0[b];
        adjacency_array.0[b] = adjacency_array.1.len() as i32;
        adjacency_array.1.push((a as i32, prev));
    }

    let (distances, max_dist) = bfs(&adjacency_array);
    let mut hides = distances
        .iter()
        .enumerate()
        .skip(2)
        .filter(|(_, &dist)| dist == max_dist);

    print!("{} {max_dist} ", hides.next().unwrap().0);
    println!("{}", hides.count() + 1);
}

fn bfs((nodes, edges): &(Vec<i32>, Vec<(i32, i32)>)) -> (Vec<u16>, u16) {
    let mut distances = vec![u16::MAX; nodes.len()];
    distances[1] = 0;

    let mut max_dist = 0;
    let mut queue = VecDeque::from([(1, 0)]);

    while let Some((node, dist)) = queue.pop_front() {
        max_dist = dist.max(max_dist);

        let new_dist = dist + 1;
        let mut edge = nodes[node as usize];

        while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
            if distances[adj as usize] == u16::MAX {
                distances[adj as usize] = new_dist;
                queue.push_back((adj, new_dist));
            }

            edge = next_edge;
        }
    }

    (distances, max_dist)
}
