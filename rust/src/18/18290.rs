use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (height, width, k) = (input() as usize, input() as usize, input() as usize);
    let map: Vec<Vec<_>> = (0..height)
        .map(|_| (0..width).map(|_| input()).collect())
        .collect();

    let max_sum = combinations(0, 0, k, &mut vec![vec![false; width]; height], &map, 0);

    println!("{max_sum}");
}

fn combinations(
    depth: usize,
    start: usize,
    max_depth: usize,
    selected: &mut Vec<Vec<bool>>,
    map: &[Vec<i32>],
    sum: i32,
) -> i32 {
    if depth == max_depth {
        return sum;
    }

    let (width, height) = (map[0].len(), map.len());
    let takes = (width * height) - (max_depth - 1);

    (start..depth + takes)
        .map(|i| {
            let (r, c) = (i / width, i % width);
            let adjacents = [
                (r.saturating_sub(1), c),
                (r, c.saturating_sub(1)),
                ((r + 1).min(height - 1), c),
                (r, (c + 1).min(width - 1)),
            ];

            if adjacents
                .iter()
                .any(|&(adj_r, adj_c)| selected[adj_r][adj_c])
            {
                return i32::MIN;
            }

            selected[r][c] = true;

            let result = combinations(depth + 1, i + 1, max_depth, selected, map, sum + map[r][c]);
            selected[r][c] = false;

            result
        })
        .max()
        .unwrap()
}
