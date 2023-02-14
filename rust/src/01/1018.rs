use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (n, m) = input.next().unwrap().split_once(' ').unwrap();
    let board: Vec<_> = input.map(str::as_bytes).collect();

    let mut min_paint = 32;

    for i in 0..=parse_int(n) - 8 {
        for j in 0..=parse_int(m) - 8 {
            min_paint = get_paint_count(&board, i, j).min(min_paint);
        }
    }

    println!("{min_paint}");
}

fn get_paint_count(board: &Vec<&[u8]>, i: usize, j: usize) -> i32 {
    let mut paint_start_from_black = 0;

    for y in 0..8 {
        for (x, &bw) in board[y + i].iter().skip(j).take(8).enumerate() {
            match (y % 2, x % 2, bw as char) {
                (0, 0, 'W') | (0, 1, 'B') | (1, 0, 'B') | (1, 1, 'W') => {
                    paint_start_from_black += 1
                }
                _ => (),
            };
        }
    }

    let paint_start_from_white = 64 - paint_start_from_black;

    paint_start_from_black.min(paint_start_from_white)
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
