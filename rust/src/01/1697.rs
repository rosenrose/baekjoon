use std::collections::VecDeque;
use std::io;

const MAX: usize = 100_000 * 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let [n, k] = [(); 2].map(|_| input.next().unwrap());
    let diff = n.abs_diff(k);

    let mut visited = [false; MAX];
    visited[n as usize] = true;

    let mut queue = VecDeque::from([(n, 0)]);

    while let Some((num, time)) = queue.pop_front() {
        if num == k {
            println!("{time}");
            return;
        }

        for adj in [num.saturating_sub(1), num + 1, num * 2] {
            if adj >= k + diff || visited[adj as usize] {
                continue;
            }

            visited[adj as usize] = true;
            queue.push_back((adj, time + 1));
        }
    }
}
