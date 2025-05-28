use std::io;

const SIZE_MAX: usize = 20;
const SHARK_MAX: usize = 20 * 20 + 1;
const DIRS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m, k) = (input(), input(), input() as i32);
    let mut sharks = [(0, 0); SHARK_MAX];
    let mut precedences = [[[0; 4]; 4]; SHARK_MAX];
    let mut map = [[(None, 0, 0); SIZE_MAX]; SIZE_MAX];

    for r in 0..n {
        for c in 0..n {
            let num = input();
            let is_shark = num > 0;

            if is_shark {
                sharks[num] = (r, c);
            }

            map[r][c] = (None, num, if is_shark { k } else { 0 });
        }
    }

    for ((r, c), dir) in (1..=m).map(|i| (sharks[i], input() - 1)) {
        map[r][c].0 = Some(dir);
    }
    for i in 1..=m {
        for r in 0..4 {
            for c in 0..4 {
                precedences[i][r][c] = input() - 1;
            }
        }
    }
    // println!("{map:?}\n{precedences:?}\n");
    let elapsed = simulate(&mut map[..n], &precedences[..=m], m, k);

    println!("{elapsed}");
}

fn simulate(
    map: &mut [[(Option<usize>, usize, i32); SIZE_MAX]],
    precedences: &[[[usize; 4]; 4]],
    mut shark_count: usize,
    duration: i32,
) -> i32 {
    for time in 1..=1000 {
        move_sharks(map, precedences, &mut shark_count);

        if shark_count == 1 {
            return time;
        }

        modify_scents(map, duration);
        // for r in &map {
        //     println!("{r:?}");
        // }
        // println!("");
    }

    -1
}

fn move_sharks(
    map: &mut [[(Option<usize>, usize, i32); SIZE_MAX]],
    precedences: &[[[usize; 4]; 4]],
    shark_count: &mut usize,
) {
    let n = map.len();
    let mut moved = [[(None, 0, 0); SIZE_MAX]; SIZE_MAX];

    for (r, row) in map.iter().enumerate() {
        moved[r][..n].clone_from_slice(&row[..n]);
    }

    for r in 0..n {
        for c in 0..n {
            let (dir, num, _) = map[r][c];
            let Some(dir) = dir else {
                continue;
            };

            let adjacents = precedences[num][dir].map(|prefer_dir| {
                let adj = (
                    (r as i8 + DIRS[prefer_dir].0).clamp(0, n as i8 - 1) as usize,
                    (c as i8 + DIRS[prefer_dir].1).clamp(0, n as i8 - 1) as usize,
                );

                (adj, prefer_dir)
            });

            let ((moved_r, moved_c), new_dir) = 'a: {
                for ((adj_r, adj_c), prefer_dir) in adjacents {
                    if map[adj_r][adj_c].1 == 0 {
                        break 'a ((adj_r, adj_c), prefer_dir);
                    }
                }

                for ((adj_r, adj_c), prefer_dir) in adjacents {
                    if (adj_r, adj_c) != (r, c) && map[adj_r][adj_c].1 == num {
                        break 'a ((adj_r, adj_c), prefer_dir);
                    }
                }

                ((r, c), dir)
            };

            moved[r][c].0 = None;

            let other_num = moved[moved_r][moved_c].1;

            if other_num == 0 || other_num >= num {
                moved[moved_r][moved_c].0 = Some(new_dir);
                moved[moved_r][moved_c].1 = num;
            }

            if other_num != 0 && other_num != num {
                *shark_count -= 1;
            }
        }
    }

    for (r, row) in map.iter_mut().enumerate() {
        row[..n].clone_from_slice(&moved[r][..n]);
    }
}

fn modify_scents(map: &mut [[(Option<usize>, usize, i32); SIZE_MAX]], duration: i32) {
    let n = map.len();

    for row in map {
        for (dir, num, scents) in &mut row[..n] {
            if *num == 0 {
                continue;
            }

            if dir.is_some() {
                *scents = duration;
                continue;
            }

            *scents -= 1;

            if *scents == 0 {
                *num = 0;
            }
        }
    }
}
