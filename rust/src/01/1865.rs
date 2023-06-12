use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let [n, m, w] = [(); 3].map(|_| input());
        let mut edges = vec![(0, 0, 0); m * 2 + w];

        for (i, [s, e, t]) in (0..m).map(|i| (i << 1, [(); 3].map(|_| input()))) {
            edges[i] = (s, e, t as i64);
            edges[i + 1] = (e, s, t as i64);
        }
        for i in 0..w {
            edges[i + m * 2] = (input(), input(), -(input() as i64))
        }

        println!("{}", if bellman_ford(n, &edges) { "YES" } else { "NO" });
    }
}

fn bellman_ford(len: usize, edges: &[(usize, usize, i64)]) -> bool {
    let mut distances = vec![0; len + 1]; // 모든 정점에서 동시에 시작. https://www.acmicpc.net/board/view/72995

    for i in 0..len {
        for &(start, end, weight) in edges {
            let new_dist = distances[start] + weight;

            if distances[end] <= new_dist {
                continue;
            }

            if i == len - 1 {
                return true;
            }

            distances[end] = new_dist;
        }
    }

    false
}
