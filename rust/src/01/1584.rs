use std::collections::VecDeque;
use std::io;

const ZONE_MAX: usize = 50;
const SIZE_MAX: usize = 500;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let danger_zones_len = input() as usize;
    let mut danger_zones = [[0; 4]; ZONE_MAX];

    for i in 0..danger_zones_len {
        danger_zones[i] = get_sorted_coords([(); 4].map(|_| input()));
    }

    let kill_zones_len = input() as usize;
    let mut kill_zones = [[0; 4]; ZONE_MAX];

    for i in 0..kill_zones_len {
        kill_zones[i] = get_sorted_coords([(); 4].map(|_| input()));
    }

    let mut visited = [[false; SIZE_MAX + 1]; SIZE_MAX + 1];
    visited[0][0] = true;

    let mut min_count = i32::MAX;
    let mut queue = VecDeque::from([((0, 0), 0)]);

    while let Some(((r, c), loss_count)) = queue.pop_front() {
        if (r, c) == (SIZE_MAX as i32, SIZE_MAX as i32) {
            min_count = loss_count.min(min_count);
            continue;
        }

        let adjacents = [
            ((r - 1).max(0), c),
            (r, (c - 1).max(0)),
            ((r + 1).min(SIZE_MAX as i32), c),
            (r, (c + 1).min(SIZE_MAX as i32)),
        ];

        for (adj_r, adj_c) in adjacents {
            if visited[adj_r as usize][adj_c as usize]
                || kill_zones[..kill_zones_len]
                    .iter()
                    .any(|&kill| is_point_inside_rect((adj_r, adj_c), kill))
            {
                continue;
            }

            visited[adj_r as usize][adj_c as usize] = true;

            if danger_zones[..danger_zones_len]
                .iter()
                .any(|&danger| is_point_inside_rect((adj_r, adj_c), danger))
            {
                queue.push_back(((adj_r, adj_c), loss_count + 1));
            } else {
                queue.push_front(((adj_r, adj_c), loss_count));
            }
        }
    }

    println!("{}", if min_count == i32::MAX { -1 } else { min_count });
}

fn get_sorted_coords([x1, y1, x2, y2]: [i32; 4]) -> [i32; 4] {
    [x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2)]
}

fn is_point_inside_rect((r, c): (i32, i32), [x1, y1, x2, y2]: [i32; 4]) -> bool {
    y1 <= r && r <= y2 && x1 <= c && c <= x2
}
