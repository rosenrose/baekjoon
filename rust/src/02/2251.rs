fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a_max, b_max, c_max] = parse_int_vec(&buf)[..] else { return };
    let mut visited = vec![vec![vec![false; c_max + 1]; c_max + 1]; c_max + 1];
    visited[0][0][c_max] = true;

    let mut c_waters = Vec::new();
    let mut stack = vec![(0, 0, c_max)];

    while let Some((a, b, c)) = stack.pop() {
        if a == 0 {
            c_waters.push(c);
        }

        let (a_spare, b_spare, c_spare) = (a_max - a, b_max - b, c_max - c);
        let adjacents = [
            (a.saturating_sub(b_spare), (b + a).min(b_max), c),
            (a.saturating_sub(c_spare), b, (c + a).min(c_max)),
            ((a + b).min(a_max), b.saturating_sub(a_spare), c),
            (a, b.saturating_sub(c_spare), (c + b).min(c_max)),
            ((a + c).min(a_max), b, c.saturating_sub(a_spare)),
            (a, (b + c).min(b_max), c.saturating_sub(b_spare)),
        ];

        for adj in adjacents {
            if visited[adj.0][adj.1][adj.2] {
                continue;
            }

            visited[adj.0][adj.1][adj.2] = true;
            stack.push(adj);
        }
    }

    c_waters.sort();

    for water in c_waters {
        print!("{water} ");
    }
}

fn parse_int_vec(buf: &str) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
