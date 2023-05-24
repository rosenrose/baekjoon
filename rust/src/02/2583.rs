use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (height, width, k) = (input(), input(), input());
    let mut map = vec![vec![false; width]; height];

    for (x1, y1, x2, y2) in (0..k).map(|_| (input(), input(), input(), input())) {
        for y in y1..y2 {
            for x in x1..x2 {
                map[y][x] = true;
            }
        }
    }

    let mut areas = get_areas(&map);
    areas.sort();

    println!("{}", areas.len());

    for area in areas {
        print!("{area} ");
    }
}

fn get_areas(map: &[Vec<bool>]) -> Vec<i32> {
    let (width, height) = (map[0].len(), map.len());
    let mut areas = Vec::new();
    let mut visited = vec![vec![false; width]; height];

    let is_pass = |r: usize, c: usize, visited: &[Vec<bool>]| visited[r][c] || map[r][c];

    for y in 0..height {
        for x in 0..width {
            if is_pass(y, x, &visited) {
                continue;
            }

            visited[y][x] = true;
            let mut count = 0;
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                count += 1;

                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(height - 1), c),
                    (r, (c + 1).min(width - 1)),
                ];

                for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
                    if is_pass(adj_r, adj_c, &visited) {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            areas.push(count);
        }
    }

    areas
}
