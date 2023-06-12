use std::io;

const SIZE: usize = 9;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut board = [[0; SIZE]; SIZE];
    let mut empty_cells = Vec::new();
    let [mut row_nums, mut col_nums, mut square_nums] = [[[false; SIZE + 1]; SIZE]; 3];

    for r in 0..SIZE {
        for (c, num) in input.by_ref().take(SIZE).enumerate() {
            if num == 0 {
                empty_cells.push((r, c));
            } else {
                [
                    row_nums[r][num],
                    col_nums[c][num],
                    square_nums[get_square_idx(r, c)][num],
                ] = [true; 3];
            }

            board[r][c] = num as u8;
        }
    }

    sudoku(
        0,
        &empty_cells,
        &mut board,
        &mut row_nums,
        &mut col_nums,
        &mut square_nums,
    );

    for row in board {
        for num in row {
            print!("{num} ");
        }
        println!("");
    }
}

fn sudoku(
    depth: usize,
    empty_cells: &[(usize, usize)],
    board: &mut [[u8; SIZE]],
    row_nums: &mut [[bool; SIZE + 1]],
    col_nums: &mut [[bool; SIZE + 1]],
    square_nums: &mut [[bool; SIZE + 1]],
) -> bool {
    if depth == empty_cells.len() {
        return true;
    }

    let (r, c) = empty_cells[depth];
    let square_idx = get_square_idx(r, c);

    for num in 1..=9 {
        if row_nums[r][num] || col_nums[c][num] || square_nums[square_idx][num] {
            continue;
        }

        board[r][c] = num as u8;
        [
            row_nums[r][num],
            col_nums[c][num],
            square_nums[square_idx][num],
        ] = [true; 3];

        let is_finished = sudoku(
            depth + 1,
            empty_cells,
            board,
            row_nums,
            col_nums,
            square_nums,
        );

        if is_finished {
            return true;
        }

        [
            row_nums[r][num],
            col_nums[c][num],
            square_nums[square_idx][num],
        ] = [false; 3];
    }

    board[r][c] = 0;

    false
}

fn get_square_idx(r: usize, c: usize) -> usize {
    (r / 3) * 3 + (c / 3)
}
