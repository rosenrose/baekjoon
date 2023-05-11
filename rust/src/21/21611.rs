use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input() as usize, input());
    let map: Vec<Vec<_>> = (0..n).map(|_| (0..n).map(|_| input()).collect()).collect();
    let blizzards = (0..m).map(|_| (input(), input() as usize));

    let counts = simulate(map, blizzards);

    println!("{}", counts[1] + (2 * counts[2]) + (3 * counts[3]));
}

fn simulate(mut map: Vec<Vec<i32>>, blizzards: impl Iterator<Item = (i32, usize)>) -> [i32; 4] {
    let n = map.len();
    let shark = (n / 2, n / 2);
    let snail = get_snail(n);
    let mut counts = [0; 4];

    for (dir, len) in blizzards {
        match dir {
            1 => {
                for r in shark.0 - len..shark.0 {
                    map[r][shark.1] = 0;
                }
            }
            2 => {
                for r in shark.0 + 1..shark.0 + 1 + len {
                    map[r][shark.1] = 0;
                }
            }
            3 => {
                for c in shark.1 - len..shark.1 {
                    map[shark.0][c] = 0;
                }
            }
            4 => {
                for c in shark.1 + 1..shark.1 + 1 + len {
                    map[shark.0][c] = 0;
                }
            }
            _ => unreachable!(),
        }

        loop {
            move_balls(&mut map, &snail);
            let groups = get_groups(&map, &snail);
            let is_exploded = explode_balls(&mut map, &snail, &groups, &mut counts);

            if !is_exploded {
                change_balls(&mut map, &snail, &groups);
                break;
            }
        }
        // for r in &map {
        //     println!("{r:?}");
        // }
        // println!("{counts:?}\n");
    }

    counts
}

fn get_snail(n: usize) -> Vec<(usize, usize)> {
    let half = n as i32 >> 1;
    let (mut r, mut c) = (half, half);
    let mut dir = (0, -1);
    let (mut len, mut count) = (1, 1);

    let mut snail = Vec::with_capacity(n * n);
    snail.push((r as usize, c as usize));

    loop {
        (r, c) = (r + dir.0, c + dir.1);
        count -= 1;

        if (r, c) == (0, -1) {
            return snail;
        }

        if count == 0 {
            dir = match dir {
                (0, -1) => (1, 0),
                (1, 0) => {
                    len += 1;
                    (0, 1)
                }
                (0, 1) => (-1, 0),
                (-1, 0) => {
                    len += 1;
                    (0, -1)
                }
                _ => unreachable!(),
            };
            count = len;
        }

        snail.push((r as usize, c as usize));
    }
}

fn move_balls(map: &mut Vec<Vec<i32>>, snail: &[(usize, usize)]) {
    let mut cursor = 0;

    for (i, &(r, c)) in snail.iter().enumerate().skip(1) {
        if map[r][c] == 0 {
            if cursor == 0 {
                cursor = i;
            }

            continue;
        }

        if cursor == 0 {
            continue;
        }

        let fill = snail[cursor];

        map[fill.0][fill.1] = map[r][c];
        map[r][c] = 0;
        cursor += 1;
    }
}

fn get_groups(map: &[Vec<i32>], snail: &[(usize, usize)]) -> Vec<((usize, usize), i32)> {
    let mut cursor = 0;
    let mut ball = 0;
    let mut groups = Vec::new();

    let end = 'outer: {
        for (i, &(r, c)) in snail.iter().enumerate().skip(1) {
            if map[r][c] == 0 {
                if cursor == 0 {
                    return groups;
                }

                break 'outer i;
            }

            if map[r][c] == ball {
                continue;
            }

            if cursor > 0 {
                groups.push(((cursor, i), ball));
            }

            cursor = i;
            ball = map[snail[i].0][snail[i].1];
        }

        snail.len()
    };

    groups.push(((cursor, end), ball));
    groups
}

fn explode_balls(
    map: &mut Vec<Vec<i32>>,
    snail: &[(usize, usize)],
    groups: &[((usize, usize), i32)],
    counts: &mut [i32],
) -> bool {
    let mut is_exploded = false;

    for &((start, end), ball) in groups {
        let count = end - start;

        if count < 4 {
            continue;
        }

        for remove in &snail[start..end] {
            map[remove.0][remove.1] = 0;
        }
        counts[ball as usize] += count as i32;

        is_exploded = true;
    }

    is_exploded
}

fn change_balls(
    map: &mut Vec<Vec<i32>>,
    snail: &[(usize, usize)],
    groups: &[((usize, usize), i32)],
) {
    let mut cursor = 1;

    for &((start, end), ball) in groups {
        for new_ball in [(end - start) as i32, ball] {
            let Some(change) = snail.get(cursor) else {
                return;
            };

            map[change.0][change.1] = new_ball;
            cursor += 1;
        }
    }

    for &(r, c) in &snail[cursor..] {
        map[r][c] = 0;
    }
}
