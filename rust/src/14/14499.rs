use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let [height, width, mut r, mut c, k] = [(); 5].map(|_| input());
    let mut map: Vec<Vec<_>> = (0..height)
        .map(|_| (0..width).map(|_| input()).collect())
        .collect();

    let [mut top, mut bottom, mut east, mut west, mut south, mut north] = [0; 6];

    for command in (0..k).map(|_| input()) {
        match (command, r, c) {
            (1, _, x) if x < width - 1 => {
                c += 1;
                (top, bottom, east, west) = (west, east, top, bottom);
            }
            (2, _, x) if x > 0 => {
                c -= 1;
                (top, bottom, east, west) = (east, west, bottom, top);
            }
            (3, y, _) if y > 0 => {
                r -= 1;
                (top, bottom, north, south) = (south, north, top, bottom);
            }
            (4, y, _) if y < height - 1 => {
                r += 1;
                (top, bottom, north, south) = (north, south, bottom, top);
            }
            _ => continue,
        }

        if map[r][c] == 0 {
            map[r][c] = bottom;
        } else {
            bottom = map[r][c];
            map[r][c] = 0;
        }

        writeln!(output, "{top}").unwrap();
    }

    print!("{output}");
}
