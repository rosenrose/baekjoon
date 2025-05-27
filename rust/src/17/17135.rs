use std::collections::VecDeque;
use std::io;

const WIDTH_MAX: usize = 15;
const HEIGHT_MAX: usize = 15;
const ARCHER_COUNT: usize = 3;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width, max_range] = [(); 3].map(|_| input.next().unwrap());
    let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for r in 0..height {
        for (c, num) in input.by_ref().take(width).enumerate() {
            map[r][c] = num == 1;
        }
    }

    let mut max_count = 0;

    for a in 0..width - 2 {
        for b in a + 1..width - 1 {
            for c in b + 1..width {
                let count = simulate(&map[..height], width, &[a, b, c], max_range);
                max_count = count.max(max_count);
            }
        }
    }

    println!("{max_count}");
}

fn simulate(map: &[[bool; WIDTH_MAX]], width: usize, archers: &[usize], max_range: usize) -> usize {
    let height = map.len();
    let mut temp = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in map.iter().enumerate() {
        temp[r][..width].copy_from_slice(&row[..width]);
    }

    (0..height)
        .map(|_| {
            let count = attack(&mut temp[..height], width, archers, max_range);
            move_down(&mut temp[..height], width);

            count
        })
        .sum()
}

fn attack(
    map: &mut [[bool; WIDTH_MAX]],
    width: usize,
    archers: &[usize],
    max_range: usize,
) -> usize {
    let height = map.len();
    let mut final_targets = [(0, 0); ARCHER_COUNT];
    let mut final_targets_len = 0;

    for &archer in archers {
        let mut targets = [((0, 0), 0); WIDTH_MAX * HEIGHT_MAX];
        let mut targets_len = 0;
        let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
        visited[height - 1][archer] = true;

        let mut queue = VecDeque::from([((height - 1, archer), 1)]);

        while let Some(((r, c), dist)) = queue.pop_front() {
            if dist <= max_range && map[r][c] {
                targets[targets_len] = ((r, c), dist);
                targets_len += 1;
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

        if targets_len == 0 {
            continue;
        }

        let (target_coord, _) = targets[..targets_len]
            .iter()
            .min_by_key(|((_, c), dist)| (dist, c))
            .unwrap();

        if !final_targets[..final_targets_len].contains(target_coord) {
            final_targets[final_targets_len] = *target_coord;
            final_targets_len += 1;
        }
    }

    for &(r, c) in &final_targets[..final_targets_len] {
        map[r][c] = false;
    }

    final_targets_len
}

fn move_down(map: &mut [[bool; WIDTH_MAX]], width: usize) {
    let height = map.len();

    for r in (1..height).rev() {
        for c in 0..width {
            map[r][c] = map[r - 1][c];
        }
    }

    for c in 0..width {
        map[0][c] = false;
    }
}
