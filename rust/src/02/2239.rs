use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines();

    let mut empty_cells = Vec::new();
    let mut board: Vec<Vec<_>> = input
        .enumerate()
        .map(|(i, row)| {
            row.char_indices()
                .map(|(j, c)| {
                    let num = c as u8 - '0' as u8;

                    if num == 0 {
                        empty_cells.push((i, j));
                    }

                    num
                })
                .collect()
        })
        .collect();

    sudoku(&mut board, &empty_cells, 0);

    for row in board {
        for num in row {
            print!("{num}");
        }
        println!("");
    }
}

fn sudoku(board: &mut Vec<Vec<u8>>, empty_cells: &[(usize, usize)], idx: usize) -> bool {
    if idx == empty_cells.len() {
        return true;
    }

    let (row, col) = empty_cells[idx];

    for num in 1..=9 {
        if (0..9).any(|i| board[row][i] == num || board[i][col] == num) {
            continue;
        }
        if (0..9).any(|i| board[(row / 3) * 3 + (i / 3)][(col / 3) * 3 + (i % 3)] == num) {
            continue;
        }

        board[row][col] = num;

        let is_finished = sudoku(board, empty_cells, idx + 1);

        if is_finished {
            return true;
        }
    }

    board[row][col] = 0;

    false
}
