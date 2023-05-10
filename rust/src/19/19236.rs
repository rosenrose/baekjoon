use std::io;

const SIZE: usize = 4;
const DIRS: [(i8, i8); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];
const SHARK: usize = 0;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut map = [[None; SIZE]; SIZE];

    for r in 0..SIZE {
        for c in 0..SIZE {
            let (num, dir) = (input.next().unwrap(), input.next().unwrap() - 1);
            map[r][c] = Some((num, dir));
        }
    }

    let (num, dir) = map[0][0].unwrap();
    map[0][0] = Some((SHARK, dir));
    // println!("{map:?}\");
    let max_sum = simulate(map, num);

    println!("{max_sum}");
}

fn simulate(mut map: [[Option<(usize, usize)>; 4]; 4], sum: usize) -> usize {
    move_fishes(&mut map);

    let (shark_r, shark_c, shark_dir) = 'a: {
        for (r, row) in map.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                let Some((num, dir)) = cell else {
                    continue;
                };

                if num == SHARK {
                    break 'a (r, c, dir);
                }
            }
        }
        unreachable!()
    };
    let shark_path = (1..)
        .map_while(|len| get_moved_coord((shark_r, shark_c), shark_dir, len))
        .filter(|&(r, c)| map[r][c].is_some());

    shark_path
        .map(|(next_r, next_c)| {
            let (fish_num, fish_dir) = map[next_r][next_c].unwrap();
            let mut map = map.clone();

            map[next_r][next_c] = Some((SHARK, fish_dir));
            map[shark_r][shark_c] = None;

            simulate(map, sum + fish_num)
        })
        .max()
        .unwrap_or(sum)
}

fn move_fishes(map: &mut [[Option<(usize, usize)>; 4]; 4]) {
    'outer: for fish_num in 1..=SIZE * SIZE {
        let (r, c, mut dir) = 'a: {
            for (r, row) in map.iter().enumerate() {
                for (c, &cell) in row.iter().enumerate() {
                    let Some((num, dir)) = cell else {
                        continue;
                    };

                    if num == fish_num {
                        break 'a (r, c, dir);
                    }
                }
            }

            continue 'outer;
        };

        let (moved_r, moved_c) = loop {
            if let Some(moved) = get_moved_coord((r, c), dir, 1) {
                if !matches!(map[moved.0][moved.1], Some((SHARK, _))) {
                    break moved;
                }
            }

            dir = (dir + 1) % DIRS.len();
        };

        map[r][c] = Some((fish_num, dir));
        (map[r][c], map[moved_r][moved_c]) = (map[moved_r][moved_c], map[r][c]);
        // for r in map.iter() {
        //     println!("{r:?}");
        // }
        // println!("");
    }
}

fn get_moved_coord((r, c): (usize, usize), dir: usize, len: i8) -> Option<(usize, usize)> {
    let moved = (r as i8 + (DIRS[dir].0 * len), c as i8 + (DIRS[dir].1 * len));

    matches!(moved, (0..=3, 0..=3)).then_some((moved.0 as usize, moved.1 as usize))
}
