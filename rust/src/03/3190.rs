use std::collections::VecDeque;
use std::io;

#[derive(Clone, Copy, Debug)]
enum Cells {
    Empty,
    Snake,
    Apple,
}

const MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n = parse_int(input());
    let mut board = [[Cells::Empty; MAX]; MAX];
    board[0][0] = Cells::Snake;

    for [r, c] in (0..parse_int(input())).map(|_| [(); 2].map(|_| parse_int(input()))) {
        board[r - 1][c - 1] = Cells::Apple;
    }

    let rotates = (0..parse_int(input())).map(|_| (parse_int(input()), input()));
    let time = simulate(&mut board[..n], rotates);

    println!("{time}");
}

fn simulate<'a>(
    board: &mut [[Cells; MAX]],
    rotates: impl Iterator<Item = (usize, &'a str)>,
) -> usize {
    let mut snake = VecDeque::from([(0, 0)]);
    let mut time = 0;
    let mut dir = (0, 1);

    for (rotate_time, rotate_dir) in rotates {
        while time < rotate_time {
            time += 1;
            let can_move = move_snake(board, &mut snake, dir);

            if !can_move {
                return time;
            }
        }

        dir = match (dir, rotate_dir) {
            ((-1, 0), "L") => (0, -1),
            ((-1, 0), "D") => (0, 1),
            ((1, 0), "L") => (0, 1),
            ((1, 0), "D") => (0, -1),
            ((0, -1), "L") => (1, 0),
            ((0, -1), "D") => (-1, 0),
            ((0, 1), "L") => (-1, 0),
            ((0, 1), "D") => (1, 0),
            _ => unreachable!(),
        };
    }

    loop {
        time += 1;
        let can_move = move_snake(board, &mut snake, dir);

        if !can_move {
            return time;
        }
    }
}

fn move_snake(board: &mut [[Cells; MAX]], snake: &mut VecDeque<(i8, i8)>, dir: (i8, i8)) -> bool {
    let n = board.len() as i8;
    let head = *snake.front().unwrap();
    let next = (head.0 + dir.0, head.1 + dir.1);

    if !(0 <= next.0 && next.0 < n && 0 <= next.1 && next.1 < n) {
        return false;
    }

    let (next_r, next_c) = (next.0 as usize, next.1 as usize);
    let next_cell = board[next_r][next_c];

    match next_cell {
        Cells::Snake => return false,
        _ => {
            board[next_r][next_c] = Cells::Snake;
            snake.push_front(next);

            if let Cells::Empty = next_cell {
                let back = snake.pop_back().unwrap();
                board[back.0 as usize][back.1 as usize] = Cells::Empty;
            }
        }
    }
    // for r in board {
    //     println!("{r:?}");
    // }
    // println!("");
    true
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
