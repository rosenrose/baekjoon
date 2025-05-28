use std::io;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Cells {
    Black,
    Rainbow,
    Normal(u8),
    Empty,
}

const MAX: usize = 20;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, _m] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut map = [[Cells::Empty; MAX]; MAX];

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            map[r][c] = match num {
                -1 => Cells::Black,
                0 => Cells::Rainbow,
                _ => Cells::Normal(num as u8),
            };
        }
    }

    let sum = simulate(&mut map[..n]);

    println!("{sum}");
}

fn simulate(map: &mut [[Cells; MAX]]) -> i32 {
    let mut score = 0;

    loop {
        let (groups, groups_len) = get_block_groups(map);

        if groups_len == 0 {
            break;
        }
        // for r in &block_groups {
        //     println!("{r:?}");
        // }
        let (count, _, members) = groups[..groups_len].iter().max().unwrap();

        for &(r, c) in members {
            map[r][c] = Cells::Empty;
        }

        score += count * count;

        move_down(map);
        rotate(map);
        move_down(map);
        // for r in &map {
        //     println!("{r:?}");
        // }
        // println!("");
    }

    score
}

fn get_block_groups(
    map: &[[Cells; MAX]],
) -> ([(i32, i32, Vec<(usize, usize)>); MAX * MAX / 2], usize) {
    let n = map.len();
    let mut visited = [[false; MAX]; MAX];
    let mut groups = [(); MAX * MAX / 2].map(|_| (0, 0, Vec::new()));
    let mut groups_len = 0;

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
            let mut group = Vec::new();
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                group.push((r, c));

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
                groups[groups_len] = (count, rainbow_count, group);
                groups_len += 1;
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

    (groups, groups_len)
}

fn move_down(map: &mut [[Cells; MAX]]) {
    let n = map.len();

    for r in (0..n - 1).rev() {
        for c in 0..n {
            if matches!(map[r][c], Cells::Black | Cells::Empty) {
                continue;
            }

            let down = (r + 1..n).find(|&r| map[r][c] != Cells::Empty).unwrap_or(n);

            (map[down - 1][c], map[r][c]) = (map[r][c], map[down - 1][c]);
        }
    }
}

fn rotate(map: &mut [[Cells; MAX]]) {
    let n = map.len();
    let mut rotated = [[Cells::Empty; MAX]; MAX];

    for r in 0..n {
        for c in 0..n {
            rotated[r][c] = map[c][n - 1 - r];
        }
    }

    for (r, row) in map.iter_mut().enumerate() {
        row[..n].copy_from_slice(&rotated[r][..n]);
    }
}
