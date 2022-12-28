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

    let adjacency_list = (0..m).fold(vec![Vec::new(); n + 1], |mut acc, _| {
        let (a, b, c) = (input() as usize, input() as usize, input());
        acc[a].push((b, c));

        acc
    });

    let (distances, is_negative_cycle) = bellman_ford(&adjacency_list, 1);

    if is_negative_cycle {
        println!("-1");
        return;
    }

    for &dist in distances.iter().skip(2) {
        writeln!(output, "{}", if dist == i64::MAX { -1 } else { dist }).unwrap();
    }

    print!("{output}");
}

fn bellman_ford(graph: &Vec<Vec<(usize, i64)>>, start: usize) -> (Vec<i64>, bool) {
    let mut distances = vec![i64::MAX; graph.len()];
    distances[start] = 0;

    for count in 1..graph.len() {
        for (node, edges) in graph.iter().enumerate().skip(1) {
            if distances[node] == i64::MAX {
                continue;
            }

            for &(neighbor, weight) in edges {
                let new_dist = distances[node] + weight;

                if distances[neighbor] <= new_dist {
                    continue;
                }

                if count == graph.len() - 1 {
                    return (distances, true);
                }

                distances[neighbor] = new_dist;
            }
        }
    }

    (distances, false)
}
