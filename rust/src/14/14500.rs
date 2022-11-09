fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

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
    let size = parse_int_vec(&buf);
    let (n, m) = (size[0], size[1]);

    let paper = parse_matrix(&mut buf, n);
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_matrix(buf: &mut String, rows: usize) -> Vec<Vec<usize>> {
    (0..rows)
        .map(|_| {
            read_line(buf);
            parse_int_vec(buf)
        })
        .collect()
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
