use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, mut k] = parse_int_vec(&buf)[..] else {
        return;
    };
    const MAX: u32 = 500_000;

    let mut visited = [[u32::MAX, u32::MAX]; MAX as usize + 1];
    visited[n as usize][0] = 0;

    let mut prev_time = 0;
    let mut queue = VecDeque::from([(n, 0)]);

    while let Some((num, time)) = queue.pop_front() {
        if time != prev_time {
            k += time;
            prev_time = time;
        }

        if k > MAX {
            break;
        }

        if num == k || visited[k as usize][time as usize & 1] <= time {
            println!("{time}");
            return;
        }

        let next_time = time + 1;

        for adj in [num.saturating_sub(1), num + 1, num * 2] {
            if adj > MAX || visited[adj as usize][next_time as usize & 1] != u32::MAX {
                continue;
            }

            visited[adj as usize][next_time as usize & 1] = next_time;
            queue.push_back((adj, next_time));
        }
    }

    println!("-1");
}

fn parse_int_vec(buf: &str) -> Vec<u32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
