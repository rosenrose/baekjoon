use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let map: Vec<Vec<_>> = (0..n)
        .map(|_| input.by_ref().take(n as usize).collect())
        .collect();

    let mut visited = vec![vec![false; n as usize]; n as usize];
    visited[0][0] = true;

    let mut stack = vec![(0, 0)];

    while let Some((r, c)) = stack.pop() {
        let num = map[r as usize][c as usize];
        let adjacents = [(r + num, c), (r, c + num)];

        for &(adj_r, adj_c) in adjacents
            .iter()
            .filter(|&&(adj_r, adj_c)| 0 <= adj_r && adj_r < n && 0 <= adj_c && adj_c < n)
        {
            if (adj_r, adj_c) == (n - 1, n - 1) {
                println!("HaruHaru");
                return;
            }

            if visited[adj_r as usize][adj_c as usize] {
                continue;
            }

            visited[adj_r as usize][adj_c as usize] = true;
            stack.push((adj_r, adj_c));
        }
    }

    println!("Hing");
}
