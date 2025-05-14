use std::collections::VecDeque;
use std::io;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Colors {
    Green,
    Red,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width, g, r] = [(); 4].map(|_| input.next().unwrap());
    let mut placeables = Vec::new();
    let map: Vec<Vec<_>> = (0..height)
        .map(|r| {
            input
                .by_ref()
                .take(width)
                .enumerate()
                .map(|(c, num)| match num {
                    0 => true,
                    1 => false,
                    2 => {
                        placeables.push((r, c));
                        false
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let max_count = combinations_green(
        0,
        0,
        &mut vec![false; placeables.len()],
        &placeables,
        g,
        r,
        &map,
    );

    println!("{max_count}");
}

fn combinations_green(
    depth: usize,
    start: usize,
    selected: &mut Vec<bool>,
    placeables: &[(usize, usize)],
    greens: usize,
    reds: usize,
    map: &[Vec<bool>],
) -> i32 {
    if depth == greens {
        let mut selected_green = Vec::with_capacity(greens);
        let mut rest_placeables = Vec::with_capacity(reds);

        for (i, &is_select) in selected.iter().enumerate() {
            if is_select {
                selected_green.push(placeables[i]);
            } else {
                rest_placeables.push(placeables[i]);
            }
        }

        return combinations_red(
            0,
            0,
            &mut vec![0; reds],
            &rest_placeables,
            map,
            &selected_green,
        );
    }

    let takes = placeables.len() - (greens - 1);

    (start..depth + takes)
        .map(|i| {
            selected[i] = true;

            let result =
                combinations_green(depth + 1, i + 1, selected, placeables, greens, reds, map);
            selected[i] = false;

            result
        })
        .max()
        .unwrap()
}

fn combinations_red(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
    placeables: &[(usize, usize)],
    map: &[Vec<bool>],
    selected_green: &[(usize, usize)],
) -> i32 {
    if depth == selected.len() {
        let selected_red: Vec<_> = selected.iter().map(|&i| placeables[i]).collect();

        return simulate(map, selected_green.to_owned(), selected_red);
    }

    let takes = placeables.len() - (selected.len() - 1);

    (start..depth + takes)
        .map(|i| {
            selected[depth] = i;
            combinations_red(depth + 1, i + 1, selected, placeables, map, selected_green)
        })
        .max()
        .unwrap()
}

fn simulate(
    map: &[Vec<bool>],
    selected_green: Vec<(usize, usize)>,
    selected_red: Vec<(usize, usize)>,
) -> i32 {
    let (width, height) = (map[0].len(), map.len());
    let mut count = 0;
    let mut visited = vec![vec![(i32::MAX, None); width]; height];
    let mut queue = VecDeque::with_capacity(selected_green.len() + selected_red.len());

    for (r, c) in selected_green {
        visited[r][c] = (0, Some(Colors::Green));
        queue.push_back((r, c));
    }
    for (r, c) in selected_red {
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
