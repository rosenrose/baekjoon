use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, k, m] = [(); 3].map(|_| input.next().unwrap());
    let mut stations = vec![Vec::new(); n + 1];
    let tubes: Vec<Vec<_>> = (0..m)
        .map(|i| {
            input
                .by_ref()
                .take(k)
                .map(|station| {
                    stations[station].push(i as i16);
                    station as i32
                })
                .collect()
        })
        .collect();
    // println!("{stations:?}\n{tubes:?}");
    let (start, end) = (1, n as i32);
    let mut min_count = i32::MAX;
    let mut visited = vec![false; stations.len()];
    visited[start as usize] = true;

    let mut queue = VecDeque::from([(start, 1)]);

    while let Some((node, count)) = queue.pop_front() {
        if node == end {
            min_count = count.min(min_count);
            continue;
        }

        for &tube in &stations[node as usize] {
            for &adj in &tubes[tube as usize] {
                if visited[adj as usize] {
                    continue;
                }

                visited[adj as usize] = true;
                queue.push_back((adj, count + 1));
            }
        }
    }

    println!("{}", if min_count == i32::MAX { -1 } else { min_count });
}
