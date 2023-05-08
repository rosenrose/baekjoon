use std::io;

const DIRS: [(i8, i8); 5] = [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m, k) = (input(), input(), input() as i32);
    let mut sharks = vec![None; m + 1];
    let mut scents = Vec::with_capacity(n * n);
    let mut precedences = vec![[[0; 4]; 4]; m + 1];

    let mut map: Vec<Vec<_>> = (0..n)
        .map(|r| {
            (0..n)
                .map(|c| {
                    let num = input();
                    let is_shark = num > 0;

                    if is_shark {
                        sharks[num] = Some(((r, c), 0));
                        scents.push((r, c));
                    }

                    (num, if is_shark { k } else { 0 })
                })
                .collect()
        })
        .collect();

    for i in 1..=m {
        let dir_idx = input();
        let ((r, c), _) = sharks[i].unwrap();

        sharks[i] = Some(((r, c), dir_idx));
    }
    for i in 1..=m {
        for r in 0..4 {
            for c in 0..4 {
                precedences[i][r][c] = input();
            }
        }
    }
    // println!("{map:?}\n{sharks:?}\n{scents:?}\n{precedences:?}");
    let elapsed = simulate(&mut map, &mut sharks, &mut scents, &precedences, k);

    println!("{elapsed}");
}

fn simulate(
    map: &mut Vec<Vec<(usize, i32)>>,
    sharks: &mut Vec<Option<((usize, usize), usize)>>,
    scents: &mut Vec<(usize, usize)>,
    precedences: &[[[usize; 4]; 4]],
    duration: i32,
) -> i32 {
    let mut shark_count = sharks.len() - 1;

    for time in 1..=1000 {
        move_sharks(map, sharks, precedences, &mut shark_count);

        if shark_count == 1 {
            return time;
        }

        modify_scents(map, sharks, scents, duration);
        // for r in map.iter() {
        //     println!("{r:?}");
        // }
        // println!("{sharks:?}\n{scents:?}\n");
    }

    -1
}

fn move_sharks(
    map: &mut Vec<Vec<(usize, i32)>>,
    sharks: &mut Vec<Option<((usize, usize), usize)>>,
    precedences: &[[[usize; 4]; 4]],
    shark_count: &mut usize,
) {
    let n = map.len();

    for num in 1..sharks.len() {
        let Some(((r, c), dir_idx)) = sharks[num] else {
            continue;
        };

        let adjacents: Vec<_> = precedences[num][dir_idx - 1]
            .iter()
            .filter_map(|&prefer_dir_idx| {
                let dir = DIRS[prefer_dir_idx];
                let adj = (
                    (r as i8 + dir.0).clamp(0, n as i8 - 1) as usize,
                    (c as i8 + dir.1).clamp(0, n as i8 - 1) as usize,
                );

                (adj != (r, c)).then_some((adj, prefer_dir_idx))
            })
            .collect();

        let ((moved_r, moved_c), new_dir_idx) = 'next: {
            for &((adj_r, adj_c), prefer_dir_idx) in adjacents.iter() {
                if map[adj_r][adj_c].0 == 0 {
                    break 'next ((adj_r, adj_c), prefer_dir_idx);
                }
            }

            for ((adj_r, adj_c), prefer_dir_idx) in adjacents {
                if map[adj_r][adj_c].0 == num {
                    break 'next ((adj_r, adj_c), prefer_dir_idx);
                }
            }

            ((r, c), dir_idx)
        };

        sharks[num] = Some(((moved_r, moved_c), new_dir_idx));
    }

    for num in 1..sharks.len() {
        let Some(((moved_r, moved_c), _)) = sharks[num] else {
            continue;
        };

        let other_num = map[moved_r][moved_c].0;

        if other_num == 0 || other_num == num {
            map[moved_r][moved_c].0 = num;
        } else {
            sharks[num] = None;
            *shark_count -= 1;
        }
    }
}

fn modify_scents(
    map: &mut Vec<Vec<(usize, i32)>>,
    sharks: &[Option<((usize, usize), usize)>],
    scents: &mut Vec<(usize, usize)>,
    duration: i32,
) {
    scents.retain(|&(r, c)| {
        map[r][c].1 -= 1;

        if map[r][c].1 == 0 {
            map[r][c].0 = 0;
        }

        map[r][c].1 > 0
    });

    for (num, &coord) in sharks.iter().enumerate().skip(1) {
        let Some(((r, c), _)) = coord else {
            continue;
        };

        if map[r][c].1 == 0 {
            scents.push((r, c));
        }

        map[r][c] = (num, duration);
    }
}
