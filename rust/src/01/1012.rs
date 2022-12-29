use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        input();
        input();

        let k = input();
        let plant_coords: HashSet<_> = (0..k).map(|_| (input(), input())).collect();

        let mut visited = HashSet::new();
        let mut count = 0;

        for &coord in plant_coords.iter() {
            if visited.contains(&coord) {
                continue;
            }

            let mut stack = vec![coord];

            while let Some((x, y)) = stack.pop() {
                let adjacent = [
                    (x.saturating_sub(1), y),
                    (x, y.saturating_sub(1)),
                    (x + 1, y),
                    (x, y + 1),
                ];
                let adjacent = adjacent
                    .iter()
                    .filter(|&coord| plant_coords.contains(coord));

                for &ad in adjacent {
                    if visited.contains(&ad) {
                        continue;
                    }

                    stack.push(ad);
                    visited.insert(ad);
                }
            }

            count += 1;
        }

        println!("{count}");
    }
}
