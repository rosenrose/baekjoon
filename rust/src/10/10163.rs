use std::fmt::Write;
use std::io;

const MAX: usize = 100 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let n = input.next().unwrap();
    let mut visible_areas = [0; MAX];
    let mut board = [[0; 1001]; 1001];

    for (paper, [x, y, w, h]) in (0..n).map(|p| (p + 1, [(); 4].map(|_| input.next().unwrap()))) {
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

    for area in &visible_areas[1..=n] {
        writeln!(output, "{area}").unwrap();
    }

    print!("{output}");
}
