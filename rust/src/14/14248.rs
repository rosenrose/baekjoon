use std::io;

const MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let start = input.next_back().unwrap() - 1;
    let mut map = [0; MAX];

    for (i, num) in input.enumerate() {
        map[i] = num;
    }

    let mut count = 0;
    let mut visited = [false; MAX];
    visited[start as usize] = true;

    let mut stack = vec![start];

    while let Some(pos) = stack.pop() {
        count += 1;

        let dist = map[pos as usize];
        let adjacents = [pos - dist, pos + dist]
            .into_iter()
            .filter(|&adj| 0 <= adj && adj < n);

        for adj in adjacents {
            if visited[adj as usize] {
                continue;
            }

            visited[adj as usize] = true;
            stack.push(adj);
        }
    }

    println!("{count}");
}
