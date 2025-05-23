use std::fmt::Write;
use std::io;

const NODES_MAX: usize = 500 + 1;
const EDGES_MAX: usize = 6000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input() as usize);
    let mut edges = [(0, 0, 0); EDGES_MAX];

    for i in 0..m {
        edges[i] = (input() as usize, input() as usize, input());
    }

    let Some(distances) = bellman_ford(n, &edges[..m], 1) else {
        println!("-1");
        return;
    };

    for &dist in &distances[2..=n] {
        writeln!(output, "{}", if dist == i64::MAX { -1 } else { dist }).unwrap();
    }

    print!("{output}");
}

fn bellman_ford(
    len: usize,
    edges: &[(usize, usize, i64)],
    init: usize,
) -> Option<[i64; NODES_MAX]> {
    let mut distances = [i64::MAX; NODES_MAX];
    distances[init] = 0;

    for i in 0..len {
        for &(start, end, weight) in edges {
            if distances[start] == i64::MAX {
                continue;
            }

            let new_dist = distances[start] + weight;

            if distances[end] <= new_dist {
                continue;
            }

            if i == len - 1 {
                return None;
            }

            distances[end] = new_dist;
        }
    }

    Some(distances)
}
