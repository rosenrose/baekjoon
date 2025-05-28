use std::io;

const WIDTH_MAX: usize = 100;
const HEIGHT_MAX: usize = 10;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, k] = [(); 2].map(|_| input.next().unwrap() as usize);
    let height = ((n as f64).sqrt().ceil() as usize).max(4);

    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for (i, num) in input.enumerate() {
        map[height - 1][i] = num;
    }

    let count = simulate(&mut map[..height], n, k as u32);

    println!("{count}");
}

fn simulate(map: &mut [[i32; WIDTH_MAX]], width: usize, k: u32) -> i32 {
    let height = map.len();
    let mut count = 0;

    loop {
        let (min, max) = map[height - 1][..width]
            .iter()
            .fold((i32::MAX, 0), |acc, &fish| {
                (acc.0.min(fish), acc.1.max(fish))
            });

        if min.abs_diff(max) <= k {
            return count;
        }

        for fish in map[height - 1][..width]
            .iter_mut()
            .filter(|fish| **fish == min)
        {
            *fish += 1;
        }

        let (start, size) = levitate_first(map, width);
        move_fishes(map, width, start, size);
        flatten(map, start, size);

        let (start, size) = levitate_second(map, width);
        move_fishes(map, width, start, size);
        flatten(map, start, size);
        // for r in &map {
        //     println!("{r:?}");
        // }
        // println!("{start:?} {size:?}");
        count += 1;
    }
}

fn levitate_first(
    map: &mut [[i32; WIDTH_MAX]],
    map_width: usize,
) -> ((usize, usize), (usize, usize)) {
    let map_height = map.len();
    let (mut width, mut height) = (1, 1);
    let mut rest = map_width - height;

    loop {
        let (y, x) = (map_height - (width + 1), map_width - rest);

        for (i, c) in (map_width - (rest + width)..map_width - rest).enumerate() {
            for (j, r) in (map_height - height..map_height).rev().enumerate() {
                map[y + i][x + j] = map[r][c];
                map[r][c] = 0;
            }
        }
        // for r in &map[map.len() - (width + 1)..] {
        //     println!("{:?}", &r[n - rest..]);
        // }
        // println!("{width} {height} {rest}");
        rest -= height;
        (width, height) = (height, width);

        if height + 1 > rest {
            return ((y, x), (width, height));
        }

        height += 1;
    }
}

fn move_fishes(
    map: &mut [[i32; WIDTH_MAX]],
    map_width: usize,
    (start_r, start_c): (usize, usize),
    (width, height): (usize, usize),
) {
    let map_height = map.len();
    let mut diffs = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for r in start_r..start_r + height {
        for c in start_c..start_c + width {
            let adjacents = [(r + 1, c), (r, (c + 1).min(start_c + width - 1))];

            for (adj_r, adj_c) in adjacents {
                let diff = (map[r][c] - map[adj_r][adj_c]) / 5;

                if diff.abs() < 1 {
                    continue;
                }

                diffs[r - start_r][c - start_c] -= diff;
                diffs[adj_r - start_r][adj_c - start_c] += diff;
            }
        }
    }
    for c in start_c..map_width - 1 {
        let right = c + 1;
        let diff = (map[map_height - 1][c] - map[map_height - 1][right]) / 5;

        if diff.abs() < 1 {
            continue;
        }

        diffs[map_height - 1 - start_r][c - start_c] -= diff;
        diffs[map_height - 1 - start_r][right - start_c] += diff;
    }

    for r in start_r..map_height {
        for c in start_c..map_width {
            map[r][c] += diffs[r - start_r][c - start_c];
        }
    }
}

fn flatten(
    map: &mut [[i32; WIDTH_MAX]],
    (start_r, start_c): (usize, usize),
    (width, height): (usize, usize),
) {
    let map_height = map.len();

    for (i, c) in (start_c..start_c + width).enumerate() {
        for (j, r) in (start_r..map_height).rev().enumerate() {
            map[map_height - 1][i * (height + 1) + j] = map[r][c];
            map[r][c] = 0;
        }
    }
}

fn levitate_second(
    map: &mut [[i32; WIDTH_MAX]],
    map_width: usize,
) -> ((usize, usize), (usize, usize)) {
    let map_height = map.len();
    let mut width = map_width;
    let (mut y, mut x) = (0, 0);

    for height in 1..=2 {
        width >>= 1;
        (y, x) = (map_height - (height * 2), map_width - width);

        for (i, r) in (map_height - height..map_height).rev().enumerate() {
            for (j, c) in (map_width - width * 2..map_width - width).rev().enumerate() {
                map[y + i][x + j] = map[r][c];
                map[r][c] = 0;
            }
        }
    }

    ((y, x), (width, 3))
}
