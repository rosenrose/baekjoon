use std::io;

const WIDTH_MAX: usize = 50;
const HEIGHT_MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.enumerate() {
        for (c, ch) in row.bytes().enumerate() {
            map[r][c] = ch;
        }
    }

    let (mut horizontal_count, mut vertical_count) = (0, 0);
    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for y in 0..height {
        for x in 0..width {
            if visited[y][x] {
                continue;
            }

            visited[y][x] = true;
            let mut stack = vec![(y, x)];

            if map[y][x] == b'-' {
                while let Some((r, c)) = stack.pop() {
                    let adjacents = [(r, c.saturating_sub(1)), (r, (c + 1).min(width - 1))];

                    for (adj_r, adj_c) in adjacents {
                        if visited[adj_r][adj_c] || map[adj_r][adj_c] == b'|' {
                            continue;
                        }

                        visited[adj_r][adj_c] = true;
                        stack.push((adj_r, adj_c));
                    }
                }

                horizontal_count += 1;
            } else {
                while let Some((r, c)) = stack.pop() {
                    let adjacents = [(r.saturating_sub(1), c), ((r + 1).min(height - 1), c)];

                    for (adj_r, adj_c) in adjacents {
                        if visited[adj_r][adj_c] || map[adj_r][adj_c] == b'-' {
                            continue;
                        }

                        visited[adj_r][adj_c] = true;
                        stack.push((adj_r, adj_c));
                    }
                }

                vertical_count += 1;
            }
        }
    }

    println!("{}", horizontal_count + vertical_count);
}
