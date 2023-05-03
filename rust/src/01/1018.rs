use std::io;

const SIZE: usize = 8;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (n, m) = input.next().unwrap().split_once(' ').unwrap();
    let (n, m) = (parse_int(n), parse_int(m));
    let board: Vec<_> = input.collect();

    let mut min_paint = SIZE * SIZE / 2;

    for y in 0..=n - SIZE {
        for x in 0..=m - SIZE {
            min_paint = get_paint_count(&board, x, y).min(min_paint);
        }
    }

    println!("{min_paint}");
}

fn get_paint_count(board: &[&str], x: usize, y: usize) -> usize {
    let mut paint_start_black = 0;

    for (i, row) in board[y..y + SIZE].iter().enumerate() {
        for (j, ch) in row[x..x + SIZE].char_indices() {
            if matches!(
                (i % 2, j % 2, ch),
                (0, 0, 'W') | (0, 1, 'B') | (1, 0, 'B') | (1, 1, 'W')
            ) {
                paint_start_black += 1
            }
        }
    }

    let paint_start_white = SIZE * SIZE - paint_start_black;

    paint_start_black.min(paint_start_white)
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}

/*
BBBWBWBW     BWBWBWBW
BBBBWBWB     WBWBWBWB
BBBWBWBW     BWBWBWBW
BBBBWBWB     WBWBWBWB
BBBWBWBW  -> BWBWBWBW
BBBBWBWB     WBWBWBWB
WWWWWBWB     BWBWBWBW
WWWWWBWB     WBWBWBWB
paint: 17

BBBWBWBW     WBWBWBWB
BBBBWBWB     BWBWBWBW
BBBWBWBW     WBWBWBWB
BBBBWBWB     BWBWBWBW
BBBWBWBW  -> WBWBWBWB
BBBBWBWB     BWBWBWBW
WWWWWBWB     WBWBWBWB
WWWWWBWB     BWBWBWBW
paint: 47
*/
