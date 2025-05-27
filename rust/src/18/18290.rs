use std::io;

const WIDTH_MAX: usize = 10;
const HEIGHT_MAX: usize = 10;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [height, width, k] = [(); 3].map(|_| input.next().unwrap() as usize);
    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for r in 0..height {
        for (c, num) in input.by_ref().take(width).enumerate() {
            map[r][c] = num;
        }
    }

    let max_sum = combinations(
        0,
        0,
        k,
        &mut [[false; WIDTH_MAX]; HEIGHT_MAX],
        &map[..height],
        width,
        0,
    );

    println!("{max_sum}");
}

fn combinations(
    depth: usize,
    start: usize,
    max_depth: usize,
    selected: &mut [[bool; WIDTH_MAX]],
    map: &[[i32; WIDTH_MAX]],
    width: usize,
    sum: i32,
) -> i32 {
    if depth == max_depth {
        return sum;
    }

    let height = map.len();
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

            let result = combinations(
                depth + 1,
                i + 1,
                max_depth,
                selected,
                map,
                width,
                sum + map[r][c],
            );
            selected[r][c] = false;

            result
        })
        .max()
        .unwrap()
}
