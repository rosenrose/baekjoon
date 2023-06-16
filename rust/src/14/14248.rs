use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let start = input.next_back().unwrap() - 1;
    let map: Vec<_> = input.collect();

    let mut count = 0;
    let mut visited = vec![false; n as usize];
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
