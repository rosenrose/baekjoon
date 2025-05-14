use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else {
        return;
    };
    let diff = n.abs_diff(k);

    let mut visited = [false; 200_000];
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

fn parse_int_vec(buf: &str) -> Vec<u32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
