use std::collections::VecDeque;
use std::io;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Colors {
    Green,
    Red,
}

const WIDTH_MAX: usize = 50;
const HEIGHT_MAX: usize = 50;
const PLACEALBE_MAX: usize = 10;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width, g, r] = [(); 4].map(|_| input.next().unwrap());
    let mut placeables = [(0, 0); PLACEALBE_MAX];
    let mut placeables_len = 0;

    let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for r in 0..height {
        for (c, num) in input.by_ref().take(width).enumerate() {
            map[r][c] = match num {
                0 => true,
                1 => false,
                2 => {
                    placeables[placeables_len] = (r, c);
                    placeables_len += 1;
                    false
                }
                _ => unreachable!(),
            };
        }
    }

    let max_count = combinations_green(
        0,
        0,
        &mut [false; PLACEALBE_MAX][..placeables_len],
        &placeables[..placeables_len],
        (g, r),
        &map[..height],
        width,
    );

    println!("{max_count}");
}

fn combinations_green(
    depth: usize,
    start: usize,
    selected: &mut [bool],
    placeables: &[(usize, usize)],
    (greens, reds): (usize, usize),
    map: &[[bool; WIDTH_MAX]],
    width: usize,
) -> i32 {
    if depth == greens {
        let mut selected_green = [(0, 0); PLACEALBE_MAX];
        let mut selected_green_len = 0;
        let mut rest_placeables = [(0, 0); PLACEALBE_MAX];
        let mut rest_placeables_len = 0;

        for (i, &is_select) in selected.iter().enumerate() {
            if is_select {
                selected_green[selected_green_len] = placeables[i];
                selected_green_len += 1;
            } else {
                rest_placeables[rest_placeables_len] = placeables[i];
                rest_placeables_len += 1;
            }
        }

        return combinations_red(
            0,
            0,
            &mut [0; PLACEALBE_MAX][..reds],
            &rest_placeables[..rest_placeables_len],
            map,
            width,
            &selected_green[..selected_green_len],
        );
    }

    let takes = placeables.len() - (greens - 1);

    (start..depth + takes)
        .map(|i| {
            selected[i] = true;

            let result = combinations_green(
                depth + 1,
                i + 1,
                selected,
                placeables,
                (greens, reds),
                map,
                width,
            );
            selected[i] = false;

            result
        })
        .max()
        .unwrap()
}

fn combinations_red(
    depth: usize,
    start: usize,
    selected: &mut [usize],
    placeables: &[(usize, usize)],
    map: &[[bool; WIDTH_MAX]],
    width: usize,
    selected_green: &[(usize, usize)],
) -> i32 {
    if depth == selected.len() {
        let mut selected_red = [(0, 0); PLACEALBE_MAX];
        let mut selected_red_len = 0;

        for &mut i in selected {
            selected_red[selected_red_len] = placeables[i];
            selected_red_len += 1;
        }

        return simulate(
            map,
            width,
            selected_green,
            &selected_red[..selected_red_len],
        );
    }

    let takes = placeables.len() - (selected.len() - 1);

    (start..depth + takes)
        .map(|i| {
            selected[depth] = i;
            combinations_red(
                depth + 1,
                i + 1,
                selected,
                placeables,
                map,
                width,
                selected_green,
            )
        })
        .max()
        .unwrap()
}

fn simulate(
    map: &[[bool; WIDTH_MAX]],
    width: usize,
    selected_green: &[(usize, usize)],
    selected_red: &[(usize, usize)],
) -> i32 {
    let height = map.len();
    let mut count = 0;
    let mut visited = [[(i32::MAX, None); WIDTH_MAX]; HEIGHT_MAX];
    let mut queue = VecDeque::with_capacity(selected_green.len() + selected_red.len());

    for &(r, c) in selected_green {
        visited[r][c] = (0, Some(Colors::Green));
        queue.push_back((r, c));
    }
    for &(r, c) in selected_red {
        visited[r][c] = (0, Some(Colors::Red));
        queue.push_back((r, c));
    }

    while let Some((r, c)) = queue.pop_front() {
        let (time, Some(color)) = visited[r][c] else {
            continue;
        };
        let next_time = time + 1;
        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            let (adj_time, adj_color) = visited[adj_r][adj_c];

            if adj_time <= next_time || map[adj_r][adj_c] {
                let Some(adj_color) = adj_color else {
                    continue;
                };

                if adj_time == next_time && adj_color != color {
                    count += 1;
                    visited[adj_r][adj_c] = (next_time, None);
                }

                continue;
            }

            visited[adj_r][adj_c] = (next_time, Some(color));
            queue.push_back((adj_r, adj_c));
        }
    }

    count
}
