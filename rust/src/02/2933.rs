use std::io;

const WIDTH_MAX: usize = 100;
const HEIGHT_MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.by_ref().take(height).enumerate() {
        for (c, ch) in row.char_indices() {
            map[r][c] = ch == 'x';
        }
    }

    let throws = input.skip(1).map(|s| height - s.parse::<usize>().unwrap());

    simulate(&mut map[..height], width, throws);
    print_map(&map[..height], width);
}

fn simulate(map: &mut [[bool; WIDTH_MAX]], width: usize, throws: impl Iterator<Item = usize>) {
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
        // print_map(map, width);
        while let Some((mut group, group_len, falling_height)) = get_hover_group(map, width) {
            group[..group_len].sort();

            for &(r, c) in group[..group_len].iter().rev() {
                map[r][c] = false;
                map[r + falling_height][c] = true;
            }
        }
    }
}

fn get_hover_group(
    map: &[[bool; WIDTH_MAX]],
    width: usize,
) -> Option<([(usize, usize); WIDTH_MAX * HEIGHT_MAX], usize, usize)> {
    let height = map.len();
    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
    let is_pass = |r: usize, c: usize, visited: &[[bool; WIDTH_MAX]]| visited[r][c] || !map[r][c];

    for y in 0..height {
        for x in 0..width {
            if is_pass(y, x, &visited) {
                continue;
            }

            visited[y][x] = true;
            let mut is_hover = true;
            let mut group = [(0, 0); WIDTH_MAX * HEIGHT_MAX];
            let mut group_len = 0;
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                if r == height - 1 {
                    is_hover = false;
                }

                group[group_len] = (r, c);
                group_len += 1;

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

            for &(r, c) in group[..group_len].iter().filter(|&&(r, c)| !map[r + 1][c]) {
                let down = r + 1;
                let landing_row = (down..height).find(|&row| map[row][c]).unwrap_or(height);

                if group[..group_len].contains(&(landing_row, c)) {
                    continue;
                }

                min_gap = min_gap.min(landing_row - down);
            }

            return Some((group, group_len, min_gap));
        }
    }

    None
}

fn print_map(map: &[[bool; WIDTH_MAX]], width: usize) {
    for row in map {
        println!(
            "{}",
            String::from_iter(row[..width].iter().map(|&b| if b { 'x' } else { '.' }))
        );
    }

    println!("");
}
