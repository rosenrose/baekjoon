use std::io;

const HEIGHT_MAX: usize = 50;
const SIZE: usize = 8;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut board = [""; HEIGHT_MAX];

    for (r, row) in input.enumerate() {
        board[r] = row;
    }

    let min_paint = (0..=height - SIZE)
        .flat_map(|y| (0..=width - SIZE).map(move |x| get_paint_count(&board[..height], x, y)))
        .min()
        .unwrap();

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
