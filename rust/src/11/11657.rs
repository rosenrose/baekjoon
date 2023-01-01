use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input());
    let edges: Vec<_> = (0..m)
        .map(|_| (input() as usize, input() as usize, input()))
        .collect();

    let (distances, is_negative_cycle) = bellman_ford(n, &edges, 1);

    if is_negative_cycle {
        println!("-1");
        return;
    }

    for &dist in distances.iter().skip(2) {
        writeln!(output, "{}", if dist == i64::MAX { -1 } else { dist }).unwrap();
    }

    print!("{output}");
}

fn bellman_ford(
    node_num: usize,
    edges: &Vec<(usize, usize, i64)>,
    start: usize,
) -> (Vec<i64>, bool) {
    let mut distances = vec![i64::MAX; node_num + 1];
    distances[start] = 0;

    for count in 0..node_num {
        for &(start, end, weight) in edges {
            if distances[start] == i64::MAX {
                continue;
            }

            let new_dist = distances[start] + weight;

            if distances[end] <= new_dist {
                continue;
            }

            if count == node_num - 1 {
                return (distances, true);
            }

            distances[end] = new_dist;
        }
    }

    (distances, false)
}
