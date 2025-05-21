use std::collections::VecDeque;
use std::io;

const MAX: usize = 30;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    'outer: while let (height @ 1.., rows @ 1.., cols @ 1..) =
        (parse_int(input()), parse_int(input()), parse_int(input()))
    {
        let (mut start, mut end) = ((0, 0, 0), (0, 0, 0));
        let mut map = [[[false; MAX]; MAX]; MAX];

        for h in 0..height {
            for r in 0..rows {
                for (c, ch) in input().char_indices() {
                    map[h][r][c] = match ch {
                        'S' => {
                            start = (h, r, c);
                            false
                        }
                        'E' => {
                            end = (h, r, c);
                            false
                        }
                        '#' => true,
                        '.' => false,
                        _ => unreachable!(),
                    };
                }
            }
        }

        let mut visited = [[[false; MAX]; MAX]; MAX];
        visited[start.0][start.1][start.2] = true;

        let mut queue = VecDeque::from([(start, 0)]);

        while let Some(((h, r, c), time)) = queue.pop_front() {
            let next_time = time + 1;
            let adjacents = [
                (h.saturating_sub(1), r, c),
                (h, r.saturating_sub(1), c),
                (h, r, c.saturating_sub(1)),
                ((h + 1).min(height - 1), r, c),
                (h, (r + 1).min(rows - 1), c),
                (h, r, (c + 1).min(cols - 1)),
            ];

            for adj in adjacents {
                if adj == end {
                    println!("Escaped in {next_time} minute(s).");
                    continue 'outer;
                }

                if visited[adj.0][adj.1][adj.2] || map[adj.0][adj.1][adj.2] {
                    continue;
                }

                visited[adj.0][adj.1][adj.2] = true;
                queue.push_back((adj, next_time));
            }
        }

        println!("Trapped!");
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
