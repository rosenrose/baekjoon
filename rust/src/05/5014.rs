use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [total_floor, start, startlink, up, down] = parse_int_vec(&buf)[..] else {
        return;
    };
    let mut visited = vec![false; total_floor as usize + 1];
    visited[start as usize] = true;

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((floor, count)) = queue.pop_front() {
        if floor == startlink {
            println!("{count}");
            return;
        }

        for &adj in [floor + up, floor - down]
            .iter()
            .filter(|&&adj| 1 <= adj && adj <= total_floor)
        {
            if visited[adj as usize] {
                continue;
            }

            visited[adj as usize] = true;
            queue.push_back((adj, count + 1));
        }
    }

    println!("use the stairs");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
