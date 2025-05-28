use std::io;

const MAX: usize = 50;
const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m, k] = [(); 3].map(|_| input.next().unwrap());
    let mut map = [(); MAX].map(|_| [(); MAX].map(|_| Vec::new()));

    for [r, c, m, s, d] in (0..m).map(|_| [(); 5].map(|_| input.next().unwrap())) {
        map[r - 1][c - 1].push((m as i32, s as i32, d));
    }

    let sum = simulate(&mut map[..n], k as i32);

    println!("{sum}");
}

fn simulate(map: &mut [[Vec<(i32, i32, usize)>; MAX]], k: i32) -> i32 {
    let n = map.len();

    for _ in 0..k {
        let mut moved = [(); MAX].map(|_| [(); MAX].map(|_| Vec::new()));

        move_fireballs(map, &mut moved[..n]);
        split_fireballs(map, &moved[..n]);
        // for r in &map {
        //     println!("{r:?}");
        // }
    }

    map.iter().flatten().flatten().map(|(m, ..)| m).sum()
}

fn move_fireballs(
    map: &mut [[Vec<(i32, i32, usize)>; MAX]],
    moved: &mut [[Vec<(i32, i32, usize)>; MAX]],
) {
    let n = map.len();

    for (r, row) in map.iter_mut().enumerate() {
        for (c, fireballs) in row[..n].iter_mut().enumerate() {
            while let Some((m, s, d)) = fireballs.pop() {
                let dir = DIRS[d];
                let (moved_r, moved_c) = (
                    (r as i32 + dir.0 * s).rem_euclid(n as i32) as usize,
                    (c as i32 + dir.1 * s).rem_euclid(n as i32) as usize,
                );

                moved[moved_r][moved_c].push((m, s, d));
            }
        }
    }
}

fn split_fireballs(
    map: &mut [[Vec<(i32, i32, usize)>; MAX]],
    moved: &[[Vec<(i32, i32, usize)>; MAX]],
) {
    let n = map.len();

    for (r, row) in moved.iter().enumerate() {
        for (c, fireballs) in row[..n].iter().enumerate() {
            if fireballs.len() == 1 {
                map[r][c].push(fireballs[0]);
                continue;
            }

            let (mut mass, mut speed) = (0, 0);
            let (mut is_all_even, mut is_all_odd) = (true, true);
            let len = moved[r][c].len() as i32;

            for (m, s, d) in fireballs {
                mass += m;
                speed += s;

                if d & 1 == 0 {
                    is_all_odd = false;
                } else {
                    is_all_even = false;
                }
            }

            mass /= 5;

            if mass == 0 {
                continue;
            }

            speed /= len;

            for i in 0..4 {
                map[r][c].push((mass, speed, (i * 2) + !(is_all_even || is_all_odd) as usize));
            }
        }
    }
}
