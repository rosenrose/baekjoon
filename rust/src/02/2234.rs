use std::io;

const WIDTH_MAX: usize = 50;
const HEIGHT_MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [width, height] = [(); 2].map(|_| input.next().unwrap());
    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for r in 0..height {
        for (c, num) in input.by_ref().take(width).enumerate() {
            map[r][c] = num as u8;
        }
    }

    let (room_count, max_area) = get_room_count_max_area(&map, (width, height));

    let mut max_area_with_break = 0;
    let dirs = [1 << 0, 1 << 1, 1 << 2, 1 << 3];
    let [left, up, right, down] = dirs;

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 0 {
                continue;
            }

            for dir in dirs {
                if map[y][x] & dir == 0
                    || (y == 0 && dir == up)
                    || (y == height - 1 && dir == down)
                    || (x == 0 && dir == left)
                    || (x == width - 1 && dir == right)
                {
                    continue;
                }

                let broken_walls = map[y][x] & !dir;
                let mut area = 0;
                let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
                visited[y][x] = true;

                let mut stack = vec![(y, x)];

                while let Some((r, c)) = stack.pop() {
                    area += 1;

                    let walls = if (r, c) == (y, x) {
                        broken_walls
                    } else {
                        map[r][c]
                    };

                    for (adj_r, adj_c) in get_adjacents(r, c, width, height, walls) {
                        if visited[adj_r][adj_c] {
                            continue;
                        }

                        visited[adj_r][adj_c] = true;
                        stack.push((adj_r, adj_c));
                    }
                }

                max_area_with_break = area.max(max_area_with_break);
            }
        }
    }

    println!("{room_count}\n{max_area}\n{max_area_with_break}");
}

fn get_room_count_max_area(map: &[[u8; WIDTH_MAX]], (width, height): (usize, usize)) -> (i32, i32) {
    let mut room_count = 0;
    let mut max_area = 0;
    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for y in 0..height {
        for x in 0..width {
            if visited[y][x] {
                continue;
            }

            visited[y][x] = true;
            let mut area = 0;
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                area += 1;
                let walls = map[r][c];

                for (adj_r, adj_c) in get_adjacents(r, c, width, height, walls) {
                    if visited[adj_r][adj_c] {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            room_count += 1;
            max_area = area.max(max_area);
        }
    }

    (room_count, max_area)
}

fn get_adjacents(
    r: usize,
    c: usize,
    width: usize,
    height: usize,
    walls: u8,
) -> impl Iterator<Item = (usize, usize)> {
    [
        (r, c.saturating_sub(1)),
        (r.saturating_sub(1), c),
        (r, (c + 1).min(width - 1)),
        ((r + 1).min(height - 1), c),
    ]
    .into_iter()
    .enumerate()
    .filter_map(move |(i, adj)| (walls & (1 << i) == 0).then_some(adj))
}
