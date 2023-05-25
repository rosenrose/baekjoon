use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let danger_zones: Vec<_> = (0..input())
        .map(|_| get_sorted_coords(input(), input(), input(), input()))
        .collect();
    let kill_zones: Vec<_> = (0..input())
        .map(|_| get_sorted_coords(input(), input(), input(), input()))
        .collect();
    const SIZE: i32 = 500;

    let mut visited = [[false; SIZE as usize + 1]; SIZE as usize + 1];
    visited[0][0] = true;

    let mut min_count = i32::MAX;
    let mut queue = VecDeque::from([((0, 0), 0)]);

    while let Some(((r, c), loss_count)) = queue.pop_front() {
        if (r, c) == (SIZE, SIZE) {
            min_count = loss_count.min(min_count);
            continue;
        }

        let adjacents = [
            ((r - 1).max(0), c),
            (r, (c - 1).max(0)),
            ((r + 1).min(SIZE), c),
            (r, (c + 1).min(SIZE)),
        ];

        for (adj_r, adj_c) in adjacents {
            if visited[adj_r as usize][adj_c as usize]
                || kill_zones
                    .iter()
                    .any(|&kill| is_point_inside_rectangle((adj_r, adj_c), kill))
            {
                continue;
            }

            visited[adj_r as usize][adj_c as usize] = true;

            if danger_zones
                .iter()
                .any(|&danger| is_point_inside_rectangle((adj_r, adj_c), danger))
            {
                queue.push_back(((adj_r, adj_c), loss_count + 1));
            } else {
                queue.push_front(((adj_r, adj_c), loss_count));
            }
        }
    }

    println!("{}", if min_count == i32::MAX { -1 } else { min_count });
}

fn get_sorted_coords(x1: i32, y1: i32, x2: i32, y2: i32) -> (i32, i32, i32, i32) {
    (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2))
}

fn is_point_inside_rectangle((r, c): (i32, i32), (x1, y1, x2, y2): (i32, i32, i32, i32)) -> bool {
    y1 <= r && r <= y2 && x1 <= c && c <= x2
}
