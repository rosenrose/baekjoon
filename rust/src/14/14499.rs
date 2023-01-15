use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m, x, y, k) = (input(), input(), input(), input(), input());

    let (mut x, mut y) = (y, x);
    let mut map: Vec<Vec<_>> = (0..n).map(|_| (0..m).map(|_| input()).collect()).collect();

    let (mut top, mut bottom, mut east, mut west, mut south, mut north) = (0, 0, 0, 0, 0, 0);

    for command in (0..k).map(|_| input()) {
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

        writeln!(output, "{top}").unwrap();
    }

    print!("{output}");
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
