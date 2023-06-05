use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, k] = parse_int_vec(&buf)[..] else { return };

    const MAX: i32 = 1_000_000;
    let mut visited = vec![false; MAX as usize + 1];
    let mut queue = VecDeque::from([(a, 0)]);

    while let Some((num, count)) = queue.pop_front() {
        let next_count = count + 1;

        for adj in [num + 1, num * 2] {
            if adj == k {
                println!("{next_count}");
                return;
            }

            if adj >= MAX || visited[adj as usize] {
                continue;
            }

            visited[adj as usize] = true;
            queue.push_back((adj, next_count));
        }
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
