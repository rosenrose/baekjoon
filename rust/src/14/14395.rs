use std::collections::{HashMap, VecDeque};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [start, end] = parse_int_vec(&buf)[..] else { return };

    if start == end {
        println!("0");
        return;
    }

    const MAX: i32 = 1_000_000_000;
    let mut visited = HashMap::from([(start, None)]);
    let mut queue = VecDeque::from([start]);

    'outer: while let Some(num) = queue.pop_front() {
        let mut adjacents = [None, Some((num + num, '+')), Some((0, '-')), None];

        if let Some(square) = num.checked_mul(num) {
            adjacents[0] = Some((square, '*'));
        }
        if num != 0 {
            adjacents[3] = Some((1, '/'));
        }

        for &(adj_num, adj_op) in adjacents.iter().flatten() {
            if adj_num > MAX || visited.contains_key(&adj_num) {
                continue;
            }

            visited.insert(adj_num, Some((num, adj_op)));

            if adj_num == end {
                break 'outer;
            }

            queue.push_back(adj_num);
        }
    }

    if !visited.contains_key(&end) {
        println!("-1");
        return;
    }

    let mut path = Vec::new();
    let mut node = end;

    while let Some((parent, op)) = visited[&node] {
        path.push(op);
        node = parent;
    }

    println!("{}", String::from_iter(path.iter().rev()));
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
