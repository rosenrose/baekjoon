use std::io;

const MAX: usize = 200 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [a_max, b_max, c_max] = [(); 3].map(|_| input.next().unwrap());
    let mut visited = [[[false; MAX]; MAX]; MAX];
    visited[0][0][c_max] = true;

    let mut c_waters = [0; MAX];
    let mut c_waters_len = 0;
    let mut stack = vec![(0, 0, c_max)];

    while let Some((a, b, c)) = stack.pop() {
        if a == 0 {
            c_waters[c_waters_len] = c;
            c_waters_len += 1;
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

    c_waters[..c_waters_len].sort();

    for water in &c_waters[..c_waters_len] {
        print!("{water} ");
    }
}
