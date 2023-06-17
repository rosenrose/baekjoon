use std::collections::HashSet;
use std::io;

const SIZE: i8 = 5;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut map = [[0; SIZE as usize]; SIZE as usize];

    for cell in map.iter_mut().flatten() {
        *cell = input.next().unwrap();
    }

    let mut visited = HashSet::new();

    for y in 0..SIZE {
        for x in 0..SIZE {
            let mut stack = vec![((y, x), map[y as usize][x as usize], 0)];

            while let Some(((r, c), num, depth)) = stack.pop() {
                if depth == 5 {
                    visited.insert(num);
                    continue;
                }

                let adjacents = [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)]
                    .into_iter()
                    .filter(|&(adj_r, adj_c)| {
                        0 <= adj_r && adj_r < SIZE && 0 <= adj_c && adj_c < SIZE
                    });

                for (adj_r, adj_c) in adjacents {
                    let digit = map[adj_r as usize][adj_c as usize];

                    stack.push(((adj_r, adj_c), num * 10 + digit, depth + 1));
                }
            }
        }
    }

    println!("{}", visited.len());
}
