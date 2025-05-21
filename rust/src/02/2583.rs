use std::io;

const WIDTH_MAX: usize = 100;
const HEIGHT_MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width, k] = [(); 3].map(|_| input.next().unwrap());
    let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for [x1, y1, x2, y2] in (0..k).map(|_| [(); 4].map(|_| input.next().unwrap())) {
        for y in y1..y2 {
            for x in x1..x2 {
                map[y][x] = true;
            }
        }
    }

    let (mut areas, areas_len) = get_areas(&map[..height], width);
    areas[..areas_len].sort();

    println!("{}", areas_len);

    for area in &areas[..areas_len] {
        print!("{area} ");
    }
}

fn get_areas(
    map: &[[bool; WIDTH_MAX]],
    width: usize,
) -> ([i32; WIDTH_MAX * HEIGHT_MAX / 2], usize) {
    let height = map.len();
    let mut areas = [0; WIDTH_MAX * HEIGHT_MAX / 2];
    let mut areas_len = 0;
    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];

    let is_pass = |r: usize, c: usize, visited: &[[bool; WIDTH_MAX]]| visited[r][c] || map[r][c];

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

                for (adj_r, adj_c) in adjacents {
                    if is_pass(adj_r, adj_c, &visited) {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            areas[areas_len] = count;
            areas_len += 1;
        }
    }

    (areas, areas_len)
}
