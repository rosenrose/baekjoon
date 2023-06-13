use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width, max_range] = [(); 3].map(|_| input.next().unwrap());
    let map: Vec<Vec<_>> = (0..height)
        .map(|_| input.by_ref().take(width).map(|num| num == 1).collect())
        .collect();

    let mut max_count = 0;

    for a in 0..width - 2 {
        for b in a + 1..width - 1 {
            for c in b + 1..width {
                let count = simulate(map.clone(), &[a, b, c], max_range);
                max_count = count.max(max_count);
            }
        }
    }

    println!("{max_count}");
}

fn simulate(mut map: Vec<Vec<bool>>, archers: &[usize], max_range: usize) -> i32 {
    (0..map.len())
        .map(|_| {
            let count = attack(&mut map, archers, max_range);
            move_down(&mut map);

            count
        })
        .sum()
}

fn attack(map: &mut Vec<Vec<bool>>, archers: &[usize], max_range: usize) -> i32 {
    let (width, height) = (map[0].len(), map.len());
    let mut final_targets = Vec::with_capacity(archers.len());

    for &archer in archers {
        let mut targets = Vec::new();
        let mut visited = vec![vec![false; width]; height];
        visited[height - 1][archer] = true;

        let mut queue = VecDeque::from([((height - 1, archer), 1)]);

        while let Some(((r, c), dist)) = queue.pop_front() {
            if dist <= max_range && map[r][c] {
                targets.push(((r, c), dist));
            }

            if dist == max_range {
                continue;
            }

            let adjacents = [
                (r.saturating_sub(1), c),
                (r, c.saturating_sub(1)),
                ((r + 1).min(height - 1), c),
                (r, (c + 1).min(width - 1)),
            ];

            for (adj_r, adj_c) in adjacents {
                if visited[adj_r][adj_c] {
                    continue;
                }

                visited[adj_r][adj_c] = true;
                queue.push_back(((adj_r, adj_c), dist + 1));
            }
        }

        if targets.is_empty() {
            continue;
        }

        let (target_coord, _) = targets
            .iter()
            .min_by_key(|((_, c), dist)| (dist, c))
            .unwrap();

        if !final_targets.contains(target_coord) {
            final_targets.push(*target_coord)
        }
    }

    let count = final_targets.len() as i32;

    for (r, c) in final_targets {
        map[r][c] = false;
    }

    count
}

fn move_down(map: &mut Vec<Vec<bool>>) {
    let (width, height) = (map[0].len(), map.len());

    for r in (1..height).rev() {
        for c in 0..width {
            map[r][c] = map[r - 1][c];
        }
    }

    for c in 0..width {
        map[0][c] = false;
    }
}
