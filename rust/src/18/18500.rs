use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (height, _width) = (
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap(),
    );
    let mut map: Vec<Vec<_>> = input
        .by_ref()
        .take(height)
        .map(|row| row.chars().map(|ch| ch == 'x').collect())
        .collect();
    let throws = input.skip(1).map(|s| height - s.parse::<usize>().unwrap());

    simulate(&mut map, throws);
    print_map(&map);
}

fn simulate(map: &mut Vec<Vec<bool>>, throws: impl Iterator<Item = usize>) {
    for (i, throw_height) in throws.enumerate() {
        if i & 1 == 0 {
            if let Some(destroy) = map[throw_height].iter_mut().find(|b| **b) {
                *destroy = false;
            }
        } else {
            if let Some(destroy) = map[throw_height].iter_mut().rev().find(|b| **b) {
                *destroy = false;
            }
        }

        while let Some((mut members, falling_height)) = get_hover_group(map) {
            members.sort();

            for &(r, c) in members.iter().rev() {
                map[r][c] = false;
                map[r + falling_height][c] = true;
            }
        }
    }
}

fn get_hover_group(map: &[Vec<bool>]) -> Option<(Vec<(usize, usize)>, usize)> {
    let (width, height) = (map[0].len(), map.len());
    let mut visited = vec![vec![false; width]; height];

    let is_pass = |r: usize, c: usize, visited: &[Vec<bool>]| visited[r][c] || !map[r][c];

    for y in 0..height {
        for x in 0..width {
            if is_pass(y, x, &visited) {
                continue;
            }

            visited[y][x] = true;
            let mut is_hover = true;
            let mut group = Vec::new();
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                if r == height - 1 {
                    is_hover = false;
                }

                group.push((r, c));

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

            if !is_hover {
                continue;
            }

            let mut min_gap = usize::MAX;

            for &(r, c) in group.iter().filter(|&&(r, c)| !map[r + 1][c]) {
                let down = r + 1;
                let landing_row = (down..height).find(|&row| map[row][c]).unwrap_or(height);

                if group.contains(&(landing_row, c)) {
                    continue;
                }

                min_gap = min_gap.min(landing_row - down);
            }

            return Some((group, min_gap));
        }
    }

    None
}

fn print_map(map: &[Vec<bool>]) {
    for row in map {
        println!(
            "{}",
            String::from_iter(row.iter().map(|&b| if b { 'x' } else { '.' }))
        );
    }

    println!("");
}
