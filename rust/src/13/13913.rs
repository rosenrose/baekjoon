use std::collections::VecDeque;
use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut output = String::new();

    let [n, k] = parse_int_vec(&buf)[..] else { return };
    let diff = n.abs_diff(k);

    let mut visited = vec![None; 200_000];
    let mut queue = VecDeque::from([(n, 0)]);

    let min_time = 'a: {
        while let Some((num, time)) = queue.pop_front() {
            if num == k {
                break 'a time;
            }

            for &adj in [num.saturating_sub(1), num + 1, num * 2]
                .iter()
                .filter(|&&adj| adj != n)
            {
                if adj >= k + diff || visited[adj as usize].is_some() {
                    continue;
                }

                visited[adj as usize] = Some(num);
                queue.push_back((adj, time + 1));
            }
        }

        unreachable!()
    };

    let mut path = Vec::with_capacity(min_time as usize);
    let mut node = k;

    while let Some(parent) = visited[node as usize] {
        path.push(parent);
        node = parent;
    }

    writeln!(output, "{}", path.len()).unwrap();

    for p in path.iter().rev() {
        write!(output, "{p} ").unwrap();
    }

    writeln!(output, "{k}").unwrap();
    print!("{output}");
}

fn parse_int_vec(buf: &str) -> Vec<u32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
