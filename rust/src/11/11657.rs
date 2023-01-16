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

fn bellman_ford(len: usize, edges: &Vec<(usize, usize, i64)>, start: usize) -> (Vec<i64>, bool) {
    let mut distances = vec![i64::MAX; len + 1];
    distances[start] = 0;

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
                return (distances, true);
            }

            distances[end] = new_dist;
        }
    }

    (distances, false)
}
