use std::io;

const WIDTH_MAX: usize = 2000;
const HEIGHT_MAX: usize = 2000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let mut it = input.by_ref().take(3).flat_map(str::parse::<usize>);
    let [n, m, k] = [(); 3].map(|_| it.next().unwrap());

    let mut sum_accum = [[[0; 4]; WIDTH_MAX + 1]; HEIGHT_MAX + 1];

    for (i, row) in input.map(str::as_bytes).enumerate() {
        for (j, ch) in row.iter().enumerate() {
            sum_accum[i + 1][j + 1] = sum_accum[i + 1][j];

            match (i & 1, j & 1, ch) {
                (0, 0, b'B') | (1, 1, b'B') => sum_accum[i + 1][j + 1][0] += 1,
                (0, 1, b'B') | (1, 0, b'B') => sum_accum[i + 1][j + 1][1] += 1,
                (0, 0, b'W') | (1, 1, b'W') => sum_accum[i + 1][j + 1][2] += 1,
                (0, 1, b'W') | (1, 0, b'W') => sum_accum[i + 1][j + 1][3] += 1,
                _ => unreachable!(),
            }
        }

        for j in 0..m {
            let (cur, prev) = (sum_accum[i + 1][j + 1], sum_accum[i][j + 1]);
            sum_accum[i + 1][j + 1] = [
                cur[0] + prev[0],
                cur[1] + prev[1],
                cur[2] + prev[2],
                cur[3] + prev[3],
            ];
        }
    }
    // for r in sum_accum {
    //     println!("{r:?}");
    // }
    let k_square = (k * k) as i32;
    let mut min_paint = k_square / 2;

    for y1 in 0..=n - k {
        for x1 in 0..=m - k {
            let (y2, x2) = (y1 + k, x1 + k);
            let (right_bottom, left_bottom, right_top, left_top) = (
                sum_accum[y2][x2],
                sum_accum[y2][x1],
                sum_accum[y1][x2],
                sum_accum[y1][x1],
            );
            // println!("{right_bottom:?} {left_bottom:?} {right_top:?} {left_top:?}")
            let [black_backslash, black_slash, white_backslash, white_slash] = [
                right_bottom[0] - left_bottom[0] - right_top[0] + left_top[0],
                right_bottom[1] - left_bottom[1] - right_top[1] + left_top[1],
                right_bottom[2] - left_bottom[2] - right_top[2] + left_top[2],
                right_bottom[3] - left_bottom[3] - right_top[3] + left_top[3],
            ];

            let paint_start_black = if matches!((y1 & 1, x1 & 1), (0, 0) | (1, 1)) {
                white_backslash + black_slash
            } else {
                white_slash + black_backslash
            };
            let paint_start_white = k_square - paint_start_black;

            min_paint = paint_start_black.min(paint_start_white).min(min_paint);
        }
    }

    println!("{min_paint}");
}
