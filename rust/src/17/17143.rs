use std::io;

const WIDTH_MAX: usize = 100;
const HEIGHT_MAX: usize = 100;
const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width, m] = [(); 3].map(|_| input.next().unwrap());
    let mut map = [[(0, 0, 0); WIDTH_MAX]; HEIGHT_MAX];

    for [row, col, s, d, z] in (0..m).map(|_| [(); 5].map(|_| input.next().unwrap())) {
        map[row - 1][col - 1] = (s as i32, d as i32 - 1, z as i32);
    }

    let sum = simulate(&mut map[..height], width);

    println!("{sum}");
}

fn simulate(map: &mut [[(i32, i32, i32); WIDTH_MAX]], width: usize) -> i32 {
    let height = map.len();
    let mut sum = 0;

    for fisherman in 0..width {
        if let Some(depth) = (0..height).position(|r| map[r][fisherman].2 > 0) {
            sum += map[depth][fisherman].2;
            map[depth][fisherman] = (0, 0, 0);
        }

        move_sharks(map, width);
        // for r in &map {
        //     println!("{r:?}");
        // }
        // println!("");
    }

    sum
}

fn move_sharks(map: &mut [[(i32, i32, i32); WIDTH_MAX]], width: usize) {
    let height = map.len();
    let mut moved = [[(0, 0, 0); WIDTH_MAX]; HEIGHT_MAX];

    let reverse_dir = |dir: i32| match dir {
        0 => 1,
        1 => 0,
        2 => 3,
        3 => 2,
        _ => unreachable!(),
    };

    for (r, row) in map.iter().enumerate() {
        for (c, &(speed, mut dir, size)) in row.iter().enumerate() {
            if size == 0 {
                continue;
            }

            let (moved_r, moved_c) = 'a: {
                let (mut r, mut c) = (r, c);
                let mut s = speed;

                let mut gap = match dir {
                    0 => r,
                    1 => height - 1 - r,
                    2 => width - 1 - c,
                    3 => c,
                    _ => unreachable!(),
                } as i32;
                let mut diff = gap.min(s);

                (r, c) = get_moved_coord((r, c), dir, diff);
                s -= diff;

                if s == 0 {
                    break 'a (r, c);
                }

                dir = reverse_dir(dir);
                s %= match dir {
                    0 | 1 => (height - 1) * 2,
                    2 | 3 => (width - 1) * 2,
                    _ => unreachable!(),
                } as i32;

                gap = match dir {
                    0 | 1 => height - 1,
                    2 | 3 => width - 1,
                    _ => unreachable!(),
                } as i32;
                diff = gap.min(s);

                (r, c) = get_moved_coord((r, c), dir, diff);
                s -= diff;

                if s == 0 {
                    break 'a (r, c);
                }

                dir = reverse_dir(dir);
                diff = s;

                get_moved_coord((r, c), dir, diff)
            };

            if moved[moved_r][moved_c].2 < size {
                moved[moved_r][moved_c] = (speed, dir, size);
            }
        }
    }

    for (r, row) in map.iter_mut().enumerate() {
        row[..width].copy_from_slice(&moved[r][..width]);
    }
}

fn get_moved_coord((r, c): (usize, usize), dir: i32, diff: i32) -> (usize, usize) {
    (
        (r as i32 + DIRS[dir as usize].0 * diff) as usize,
        (c as i32 + DIRS[dir as usize].1 * diff) as usize,
    )
}
