use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);
        read_line(&mut buf);

        let mut logs = parse_int_vec(&buf);
        logs.sort();

        let mut new_logs = VecDeque::new();

        while !logs.is_empty() {
            let log = logs.pop().unwrap();

            if new_logs.len() % 2 == 0 {
                new_logs.push_back(log);
            } else {
                new_logs.push_front(log);
            }
        }

        let len = new_logs.len();
        let mut max_diff = (new_logs[0] - new_logs[len - 1]).abs();

        for i in 0..len - 1 {
            let diff = (new_logs[i] - new_logs[i + 1]).abs();

            if diff > max_diff {
                max_diff = diff;
            }
        }

        println!("{max_diff}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
