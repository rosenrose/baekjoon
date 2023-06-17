use std::io;

const WIDTH: usize = 6;
const HEIGHT: usize = 12;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(|line| line.chars());
    let mut map = [[None; WIDTH]; HEIGHT];

    for cell in map.iter_mut().flatten() {
        let ch = input.next().unwrap();
        *cell = (ch != '.').then_some(ch);
    }

    let count = simulate(map);

    println!("{count}");
}

fn simulate(mut map: [[Option<char>; WIDTH]; HEIGHT]) -> i32 {
    let mut count = 0;
    let is_pass = |r: usize, c: usize, visited: &[[bool; WIDTH]], map: &[[Option<char>; WIDTH]]| {
        visited[r][c] || map[r][c].is_none()
    };

    loop {
        let mut groups = Vec::new();
        let mut visited = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if is_pass(y, x, &visited, &map) {
                    continue;
                }

                visited[y][x] = true;
                let color = map[y][x].unwrap();
                let mut group = Vec::new();
                let mut stack = vec![(y, x)];

                while let Some((r, c)) = stack.pop() {
                    group.push((r, c));

                    let adjacents = [
                        (r.saturating_sub(1), c),
                        (r, c.saturating_sub(1)),
                        ((r + 1).min(HEIGHT - 1), c),
                        (r, (c + 1).min(WIDTH - 1)),
                    ];

                    for (adj_r, adj_c) in adjacents {
                        if is_pass(adj_r, adj_c, &visited, &map)
                            || map[adj_r][adj_c].unwrap() != color
                        {
                            continue;
                        }

                        visited[adj_r][adj_c] = true;
                        stack.push((adj_r, adj_c));
                    }
                }

                if group.len() >= 4 {
                    groups.push(group);
                }
            }
        }

        if groups.is_empty() {
            return count;
        }

        for group in groups {
            for (r, c) in group {
                map[r][c] = None;
            }
        }

        move_down(&mut map);
        count += 1;
    }
}

fn move_down(map: &mut [[Option<char>; WIDTH]]) {
    for r in (0..HEIGHT - 1).rev() {
        for c in 0..WIDTH {
            if map[r][c].is_none() {
                continue;
            }

            let down = (r + 1..HEIGHT)
                .find(|&r| map[r][c].is_some())
                .unwrap_or(HEIGHT);

            (map[down - 1][c], map[r][c]) = (map[r][c], map[down - 1][c]);
        }
    }
}
