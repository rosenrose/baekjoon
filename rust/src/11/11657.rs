use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input());
    let edges: Vec<_> = (0..m)
        .map(|_| (input() as usize, input() as usize, input()))
        .collect();

    let Some(distances) = bellman_ford(n, &edges, 1) else {
        println!("-1");
        return;
    };

    for &dist in &distances[2..] {
        writeln!(output, "{}", if dist == i64::MAX { -1 } else { dist }).unwrap();
    }

    print!("{output}");
}

fn bellman_ford(len: usize, edges: &[(usize, usize, i64)], init: usize) -> Option<Vec<i64>> {
    let mut distances = vec![i64::MAX; len + 1];
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
