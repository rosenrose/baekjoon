use std::io;

const NODES_MAX: usize = 500 + 1;
const EDGES_MAX: usize = 2500 * 2 + 200;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let [n, m, w] = [(); 3].map(|_| input());
        let mut edges = [(0, 0, 0); EDGES_MAX];

        for (i, [s, e, t]) in (0..m).map(|i| (i << 1, [(); 3].map(|_| input()))) {
            edges[i] = (s, e, t as i64);
            edges[i + 1] = (e, s, t as i64);
        }
        for i in 0..w {
            edges[i + m * 2] = (input(), input(), -(input() as i64))
        }

        println!(
            "{}",
            if bellman_ford(n, &edges, m * 2 + w) {
                "YES"
            } else {
                "NO"
            }
        );
    }
}

fn bellman_ford(nodes_len: usize, edges: &[(usize, usize, i64)], edges_len: usize) -> bool {
    let mut distances = [0; NODES_MAX]; // 모든 정점에서 동시에 시작. https://www.acmicpc.net/board/view/72995

    for i in 0..nodes_len {
        for &(start, end, weight) in &edges[..edges_len] {
            let new_dist = distances[start] + weight;

            if distances[end] <= new_dist {
                continue;
            }

            if i == nodes_len - 1 {
                return true;
            }

            distances[end] = new_dist;
        }
    }

    false
}
