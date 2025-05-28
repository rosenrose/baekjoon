use std::io;

#[derive(Copy, Clone, PartialEq)]
enum Cells {
    Empty,
    Wall,
    People,
}

const WIDTH_MAX: usize = 600;
const HEIGHT_MAX: usize = 600;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut start = (0, 0);

    let mut map = [[Cells::Empty; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.enumerate() {
        for (c, ch) in row.char_indices() {
            map[r][c] = match ch {
                'O' => Cells::Empty,
                'X' => Cells::Wall,
                'I' => {
                    start = (r, c);
                    Cells::Empty
                }
                'P' => Cells::People,
                _ => unreachable!(),
            };
        }
    }

    let mut count = 0;
    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
    visited[start.0][start.1] = true;

    let mut stack = vec![start];

    while let Some((r, c)) = stack.pop() {
        if let Cells::People = map[r][c] {
            count += 1;
        }

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if visited[adj_r][adj_c] || map[adj_r][adj_c] == Cells::Wall {
                continue;
            }

            visited[adj_r][adj_c] = true;
            stack.push((adj_r, adj_c));
        }
    }

    if count == 0 {
        println!("TT");
        return;
    }

    println!("{count}");
}
