use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (m, n) = (
        input.next().unwrap() as usize,
        input.next().unwrap() as usize,
    );
    let map: Vec<Vec<_>> = (0..m).map(|_| input.by_ref().take(n).collect()).collect();
    let mut memo = vec![vec![-1; n]; m];
    memo[m - 1][n - 1] = 1;

    println!("{}", get_count(0, 0, &map, &mut memo));
    // for r in &memo {
    //     println!("{r:?}");
    // }
}

fn get_count(r: usize, c: usize, map: &[Vec<i32>], memo: &mut Vec<Vec<i32>>) -> i32 {
    if memo[r][c] != -1 {
        return memo[r][c];
    }

    let adjacents = [
        (r.saturating_sub(1), c),
        (r, c.saturating_sub(1)),
        ((r + 1).min(map.len() - 1), c),
        (r, (c + 1).min(map[0].len() - 1)),
    ];
    let count: i32 = adjacents
        .iter()
        .filter_map(|&(adj_r, adj_c)| {
            ((adj_r, adj_c) != (r, c) && map[adj_r][adj_c] < map[r][c])
                .then(|| get_count(adj_r, adj_c, map, memo))
        })
        .sum();

    memo[r][c] = count;
    count
}
