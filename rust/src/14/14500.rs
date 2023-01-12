use std::io;

fn main() {
    const TETROMINOES: [[(usize, usize); 4]; 19] = [
        // I
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        // O
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        // L
        [(0, 0), (1, 0), (2, 0), (2, 1)],
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(0, 0), (0, 1), (0, 2), (1, 0)],
        [(0, 2), (1, 0), (1, 1), (1, 2)],
        // J
        [(0, 0), (0, 1), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 0), (2, 1)],
        [(0, 0), (1, 0), (1, 1), (1, 2)],
        [(0, 0), (0, 1), (0, 2), (1, 2)],
        // S
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(0, 1), (0, 2), (1, 0), (1, 1)],
        // Z
        [(0, 1), (1, 0), (1, 1), (2, 0)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
        // T
        [(0, 0), (0, 1), (0, 2), (1, 1)],
        [(0, 1), (1, 0), (1, 1), (1, 2)],
        [(0, 0), (1, 0), (1, 1), (2, 0)],
        [(0, 1), (1, 0), (1, 1), (2, 1)],
    ];
    // print_tetrominos(TETROMINOES);
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let paper: Vec<Vec<_>> = (0..n).map(|_| (0..m).map(|_| input()).collect()).collect();

    let mut max_sum = 0;

    for (i, row) in paper.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            'mino: for tetro in TETROMINOES {
                let mut sum = 0;

                for (y, x) in tetro {
                    if i + y > n - 1 || j + x > m - 1 {
                        continue 'mino;
                    }

                    sum += paper[i + y][j + x];
                }

                max_sum = sum.max(max_sum);
            }
        }
    }

    println!("{max_sum}");
}

fn print_tetrominos(tetrominos: [[(usize, usize); 4]; 19]) {
    for tetro in tetrominos {
        let mut paper = [['□'; 4]; 4];

        for (y, x) in tetro {
            paper[y][x] = '■';
        }

        for row in paper {
            println!("{}", String::from_iter(row));
        }
        println!("");
    }
}
