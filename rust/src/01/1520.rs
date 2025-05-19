use std::io;

const WIDTH_MAX: usize = 500;
const HEIGHT_MAX: usize = 500;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [height, width] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for r in 0..height {
        for (c, num) in input.by_ref().take(width).enumerate() {
            map[r][c] = num;
        }
    }

    let mut memo = [[None; WIDTH_MAX]; HEIGHT_MAX];
    memo[height - 1][width - 1] = Some(1);

    println!("{}", get_count(0, 0, &map, (width, height), &mut memo));
    // for r in &memo {
    //     println!("{r:?}");
    // }
}

fn get_count(
    r: usize,
    c: usize,
    map: &[[i32; WIDTH_MAX]; HEIGHT_MAX],
    (width, height): (usize, usize),
    memo: &mut [[Option<i32>; WIDTH_MAX]; HEIGHT_MAX],
) -> i32 {
    if let Some(count) = memo[r][c] {
        return count;
    }

    let adjacents = [
        (r.saturating_sub(1), c),
        (r, c.saturating_sub(1)),
        ((r + 1).min(height - 1), c),
        (r, (c + 1).min(width - 1)),
    ];

    let count: i32 = adjacents
        .iter()
        .filter_map(|&(adj_r, adj_c)| {
            (map[adj_r][adj_c] < map[r][c])
                .then(|| get_count(adj_r, adj_c, map, (width, height), memo))
        })
        .sum();

    memo[r][c] = Some(count);
    count
}
