use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let map: Vec<_> = input.map(str::as_bytes).collect();

    let (mut horizontal_count, mut vertical_count) = (0, 0);
    let mut visited = vec![vec![false; width]; height];

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
