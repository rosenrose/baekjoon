use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let mut visited = vec![u8::MAX; n as usize + 1];
    let mut queue = VecDeque::from_iter(input.skip(1).map(|p| {
        visited[p as usize] = 0;
        (p, 0)
    }));

    const MAX_BIT: usize = 20;
    let mut max_safety = 0;

    while let Some((num, safety)) = queue.pop_front() {
        let next_safety = safety + 1;
        let adjacents = (0..MAX_BIT).filter_map(|i| {
            let adj = num ^ (1 << i);
            (adj <= n).then_some(adj)
        });

        for adj in adjacents {
            if visited[adj as usize] <= next_safety {
                continue;
            }

            visited[adj as usize] = next_safety;
            max_safety = next_safety.max(max_safety);
            queue.push_back((adj, next_safety));
        }
    }

    println!("{max_safety}");
}
