use std::cmp::Reverse;
use std::io;

#[derive(Copy, Clone, Debug)]
enum Cells {
    Black,
    Rainbow,
    Normal(u8),
    Empty,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (n, _) = (input.next().unwrap() as usize, input.next());
    let map: Vec<Vec<_>> = (0..n)
        .map(|_| {
            input
                .by_ref()
                .take(n)
                .map(|num| match num {
                    -1 => Cells::Black,
                    0 => Cells::Rainbow,
                    _ => Cells::Normal(num as u8),
                })
                .collect()
        })
        .collect();

    let sum = simulate(map);

    println!("{sum}");
}

fn simulate(mut map: Vec<Vec<Cells>>) -> i32 {
    let mut score = 0;

    loop {
        let mut block_groups = get_block_groups(&map);

        if block_groups.is_empty() {
            break;
        }
        // for r in &block_groups {
        //     println!("{r:?}");
        // }
        let (count, _, members) = block_groups
            .select_nth_unstable_by_key(0, |(count, rainbow_count, members)| {
                (
                    Reverse(*count),
                    Reverse(*rainbow_count),
                    Reverse(members[0]),
                )
            })
            .1;

        for (r, c) in members {
            map[*r][*c] = Cells::Empty;
        }

        score += *count * *count;

        move_down(&mut map);
        rotate(&mut map);
        move_down(&mut map);
        // for r in &map {
        //     println!("{r:?}");
        // }
        // println!("");
    }

    score
}

fn get_block_groups(map: &[Vec<Cells>]) -> Vec<(i32, i32, Vec<(usize, usize)>)> {
    let n = map.len();
    let mut visited = vec![vec![false; n]; n];
    let mut block_groups = Vec::new();

    for y in 0..n {
        for x in 0..n {
            if visited[y][x] {
                continue;
            }
            let Cells::Normal(color) = map[y][x] else {
                continue;
            };

            visited[y][x] = true;

            let (mut count, mut rainbow_count) = (1, 0);
            let mut members = Vec::new();
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                members.push((r, c));

                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(n - 1), c),
                    (r, (c + 1).min(n - 1)),
                ];

                for (adj_r, adj_c) in adjacents {
                    if visited[adj_r][adj_c] {
                        continue;
                    }

                    match map[adj_r][adj_c] {
                        Cells::Normal(adj_color) => {
                            if color != adj_color {
                                continue;
                            }
                        }
                        Cells::Rainbow => rainbow_count += 1,
                        _ => continue,
                    }

                    count += 1;
                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            if count > 1 {
                block_groups.push((count, rainbow_count, members));
            }

            for r in 0..n {
                for c in 0..n {
                    if let Cells::Rainbow = map[r][c] {
                        visited[r][c] = false;
                    }
                }
            }
        }
    }

    block_groups
}

fn move_down(map: &mut Vec<Vec<Cells>>) {
    let n = map.len();

    for r in (0..n - 1).rev() {
        for c in 0..n {
            if matches!(map[r][c], Cells::Black | Cells::Empty) {
                continue;
            }

            let down = (r + 1..n)
                .find(|&r| !matches!(map[r][c], Cells::Empty))
                .unwrap_or(n);

            (map[down - 1][c], map[r][c]) = (map[r][c], map[down - 1][c]);
        }
    }
}

fn rotate(map: &mut Vec<Vec<Cells>>) {
    let rotated: Vec<Vec<_>> = (0..map.len())
        .rev()
        .map(|c| (0..map.len()).map(|r| map[r][c]).collect())
        .collect();

    *map = rotated;
}
