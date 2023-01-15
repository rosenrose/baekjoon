use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut empty_cells = Vec::new();
    let mut board: Vec<Vec<_>> = (0..9)
        .map(|row| {
            (0..9)
                .map(|col| {
                    let num = input.next().unwrap();

                    if num == 0 {
                        empty_cells.push((row, col));
                    }

                    num
                })
                .collect()
        })
        .collect();

    sudoku(&mut board, &empty_cells, 0);

    for row in board {
        for num in row {
            print!("{num} ");
        }
        println!("");
    }
}

fn sudoku(board: &mut Vec<Vec<i32>>, empty_cells: &Vec<(usize, usize)>, idx: usize) -> bool {
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
