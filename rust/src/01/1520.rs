use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [height, width] = [(); 2].map(|_| input.next().unwrap() as usize);
    let map: Vec<Vec<_>> = (0..height)
        .map(|_| input.by_ref().take(width).collect())
        .collect();

    let mut memo = vec![vec![None; width]; height];
    memo[height - 1][width - 1] = Some(1);

    println!("{}", get_count(0, 0, &map, &mut memo));
    // for r in &memo {
    //     println!("{r:?}");
    // }
}

fn get_count(r: usize, c: usize, map: &[Vec<i32>], memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
    if let Some(count) = memo[r][c] {
        return count;
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
            (map[adj_r][adj_c] < map[r][c]).then(|| get_count(adj_r, adj_c, map, memo))
        })
        .sum();

    memo[r][c] = Some(count);
    count
}
