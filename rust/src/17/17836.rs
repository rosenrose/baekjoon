use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width, time_limit] = [(); 3].map(|_| input.next().unwrap());
    let mut sword = (0, 0);
    let map: Vec<Vec<_>> = (0..height)
        .map(|r| {
            input
                .by_ref()
                .take(width)
                .enumerate()
                .map(|(c, num)| match num {
                    0 => false,
                    1 => true,
                    2 => {
                        sword = (r, c);
                        false
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    if let Some(time) = simulate(map, time_limit as i32, sword) {
        println!("{time}");
    } else {
        println!("Fail");
    }
}

fn simulate(map: Vec<Vec<bool>>, time_limit: i32, sword: (usize, usize)) -> Option<i32> {
    let (width, height) = (map[0].len(), map.len());
    let start = (0, 0);
    let mut visited = vec![vec![[false; 2]; width]; height];
    visited[start.0][start.1][0] = true;

    let mut min_time = i32::MAX;
    let mut queue = VecDeque::from([(start, 0, false)]);

    while let Some(((r, c), time, has_sword)) = queue.pop_front() {
        if (r, c) == (height - 1, width - 1) {
            min_time = time.min(min_time);
            continue;
        }

        if time == time_limit {
            continue;
        }

        let next_time = time + 1;
        let has_sword_idx = has_sword as usize;
        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if visited[adj_r][adj_c][has_sword_idx] || (!has_sword && map[adj_r][adj_c]) {
                continue;
            }

            visited[adj_r][adj_c][has_sword_idx] = true;
            let found_sword = (adj_r, adj_c) == sword;

            if found_sword {
                visited[adj_r][adj_c][found_sword as usize] = true;
                queue.push_back(((adj_r, adj_c), next_time, found_sword));
            } else {
                queue.push_back(((adj_r, adj_c), next_time, has_sword));
            }
        }
    }

    (min_time != i32::MAX).then_some(min_time)
}
