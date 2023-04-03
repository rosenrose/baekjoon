use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (m, n, k) = (input(), input(), input());
        let plant_coords: HashSet<_> = (0..k).map(|_| (input(), input())).collect();

        let mut visited = HashSet::new();
        let mut count = 0;

        for &coord in plant_coords.iter() {
            if visited.contains(&coord) {
                continue;
            }

            let mut stack = vec![coord];

            while let Some((x, y)) = stack.pop() {
                let adjacents = [
                    (x.saturating_sub(1), y),
                    (x, y.saturating_sub(1)),
                    ((x + 1).min(m - 1), y),
                    (x, (y + 1).min(n - 1)),
                ];

                for &adj in adjacents.iter().filter(|adj| plant_coords.contains(adj)) {
                    if visited.contains(&adj) {
                        continue;
                    }

                    stack.push(adj);
                    visited.insert(adj);
                }
            }

            count += 1;
        }

        println!("{count}");
    }
}
