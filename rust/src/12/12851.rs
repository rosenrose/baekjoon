use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else { return };
    let diff = n.abs_diff(k);

    let mut visited = [i32::MAX; 200_000];
    visited[n as usize] = 0;

    let mut min_time = i32::MAX;
    let mut count = 0;
    let mut queue = VecDeque::from([(n, 0)]);

    while let Some((num, time)) = queue.pop_front() {
        if num == k {
            if time == min_time {
                count += 1;
            }

            if time < min_time {
                min_time = time;
                count = 1;
            }

            continue;
        }

        for adj in [num.saturating_sub(1), num + 1, num * 2] {
            let new_time = time + 1;

            if adj >= k + diff || new_time > visited[adj as usize] {
                continue;
            }

            visited[adj as usize] = new_time;
            queue.push_back((adj, new_time));
        }
    }

    println!("{min_time}\n{count}");
}

fn parse_int_vec(buf: &str) -> Vec<u32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
