use std::io;

const DIRS: [(i8, i8); 5] = [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m, k) = (input(), input(), input() as i32);
    let mut sharks = vec![(0, 0); m + 1];
    let mut precedences = vec![[[0; 4]; 4]; m + 1];

    let mut map: Vec<Vec<_>> = (0..n)
        .map(|r| {
            (0..n)
                .map(|c| {
                    let num = input();
                    let is_shark = num > 0;

                    if is_shark {
                        sharks[num] = (r, c);
                    }

                    (None, num, if is_shark { k } else { 0 })
                })
                .collect()
        })
        .collect();

    for ((r, c), dir) in (1..=m).map(|i| (sharks[i], input())) {
        map[r][c].0 = Some(dir);
    }
    for i in 1..=m {
        for r in 0..4 {
            for c in 0..4 {
                precedences[i][r][c] = input();
            }
        }
    }
    // println!("{map:?}\n{precedences:?}\n");
    let elapsed = simulate(map, precedences, m, k);

    println!("{elapsed}");
}

fn simulate(
    mut map: Vec<Vec<(Option<usize>, usize, i32)>>,
    precedences: Vec<[[usize; 4]; 4]>,
    mut shark_count: usize,
    duration: i32,
) -> i32 {
    for time in 1..=1000 {
        move_sharks(&mut map, &precedences, &mut shark_count);

        if shark_count == 1 {
            return time;
        }

        modify_scents(&mut map, duration);
        // for r in map.iter() {
        //     println!("{r:?}");
        // }
        // println!("");
    }

    -1
}

fn move_sharks(
    map: &mut Vec<Vec<(Option<usize>, usize, i32)>>,
    precedences: &[[[usize; 4]; 4]],
    shark_count: &mut usize,
) {
    let n = map.len();
    let mut moved = map.clone();

    for r in 0..n {
        for c in 0..n {
            let (dir, num, _) = map[r][c];
            let Some(dir) = dir else {
                continue;
            };

            let adjacents: Vec<_> = precedences[num][dir - 1]
                .iter()
                .filter_map(|&prefer_dir| {
                    let adj = (
                        (r as i8 + DIRS[prefer_dir].0).clamp(0, n as i8 - 1) as usize,
                        (c as i8 + DIRS[prefer_dir].1).clamp(0, n as i8 - 1) as usize,
                    );

                    (adj != (r, c)).then_some((adj, prefer_dir))
                })
                .collect();

            let ((moved_r, moved_c), new_dir) = 'a: {
                for &((adj_r, adj_c), prefer_dir) in adjacents.iter() {
                    if map[adj_r][adj_c].1 == 0 {
                        break 'a ((adj_r, adj_c), prefer_dir);
                    }
                }

                for ((adj_r, adj_c), prefer_dir) in adjacents {
                    if map[adj_r][adj_c].1 == num {
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

    *map = moved
}

fn modify_scents(map: &mut Vec<Vec<(Option<usize>, usize, i32)>>, duration: i32) {
    for row in map.iter_mut() {
        for (dir, num, scents) in row.iter_mut() {
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
