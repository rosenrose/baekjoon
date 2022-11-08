use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    if let [n, m, x, y, _] = parse_int_vec(&buf)[..] {
        let (mut x, mut y) = (y, x);
        let mut map: Vec<_> = (0..n)
            .map(|_| {
                buf.clear();
                stdin.read_line(&mut buf).unwrap();
                parse_int_vec(&buf)
            })
            .collect();

        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let (mut top, mut bottom, mut east, mut west, mut south, mut north) = (0, 0, 0, 0, 0, 0);

        for command in parse_int_vec(&buf) {
            match (command, x, y) {
                (1, i, _) if i == m - 1 => continue,
                (2, 0, _) | (3, _, 0) => continue,
                (4, _, i) if i == n - 1 => continue,
                _ => (),
            }

            match command {
                1 => {
                    x += 1;
                    (top, bottom, east, west) = (west, east, top, bottom);
                }
                2 => {
                    x -= 1;
                    (top, bottom, east, west) = (east, west, bottom, top);
                }
                3 => {
                    y -= 1;
                    (top, bottom, north, south) = (south, north, top, bottom);
                }
                4 => {
                    y += 1;
                    (top, bottom, north, south) = (north, south, bottom, top);
                }
                _ => (),
            }

            if map[y][x] == 0 {
                map[y][x] = bottom;
            } else {
                bottom = map[y][x];
                map[y][x] = 0;
            }

            writeln!(stdout, "{top}").unwrap();
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

/*
match (command, &mut x, &mut y) {
    (1, x, _) if *x < m - 1 => {
        *x += 1;
        (top, bottom, east, west) = (west, east, top, bottom);
    }
    (2, x, _) if *x > 0 => {
        *x -= 1;
        (top, bottom, east, west) = (east, west, bottom, top);
    }
    (3, _, y) if *y > 0 => {
        *y -= 1;
        (top, bottom, north, south) = (south, north, top, bottom);
    }
    (4, _, y) if *y < n - 1 => {
        *y += 1;
        (top, bottom, north, south) = (north, south, bottom, top);
    }
    _ => (),
}
*/
