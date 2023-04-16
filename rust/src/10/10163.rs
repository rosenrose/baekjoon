use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = input();
    let mut visible_areas = vec![0; n + 1];
    let mut board = [[0; 1001]; 1001];

    for (paper, (x, y, w, h)) in (0..n).map(|p| (p + 1, (input(), input(), input(), input()))) {
        for i in y..y + h {
            for j in x..x + w {
                if board[i][j] != 0 {
                    visible_areas[board[i][j]] -= 1;
                }

                visible_areas[paper] += 1;
                board[i][j] = paper;
            }
        }
    }

    for area in &visible_areas[1..] {
        writeln!(output, "{area}").unwrap();
    }

    print!("{output}");
}
