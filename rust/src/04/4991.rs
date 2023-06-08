use std::collections::VecDeque;
use std::io;

enum Cells {
    Clean,
    Dirty(u8),
    Wall,
}

const ROBOT_IDX: usize = 0;
const MAX_DIRTY: usize = 10;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    while let (_width @ 1.., height @ 1..) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    ) {
        let mut dirties = Vec::new();
        let mut robot = (0, 0);

        let map: Vec<Vec<_>> = input
            .by_ref()
            .take(height)
            .enumerate()
            .map(|(r, row)| {
                row.char_indices()
                    .map(|(c, ch)| match ch {
                        '.' => Cells::Clean,
                        '*' => {
                            dirties.push((r, c));
                            Cells::Dirty(dirties.len() as u8)
                        }
                        'x' => Cells::Wall,
                        'o' => {
                            robot = (r, c);
                            Cells::Clean
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        println!("{}", simulate(map, dirties, robot).unwrap_or(-1));
    }
}

fn simulate(
    map: Vec<Vec<Cells>>,
    dirties: Vec<(usize, usize)>,
    robot: (usize, usize),
) -> Option<i32> {
    let dist_matrix = get_dists(&map, &dirties, robot);

    if dist_matrix[..dirties.len()]
        .iter()
        .flat_map(|row| &row[..dirties.len()])
        .any(|&dist| dist == i32::MAX)
    {
        return None;
    }

    let mut min_dist = i32::MAX;

    permutations(
        0,
        &mut vec![0; dirties.len()],
        &mut [false; MAX_DIRTY],
        &dist_matrix,
        robot,
        &mut min_dist,
    );

    (min_dist != i32::MAX).then_some(min_dist)
}

fn get_dists(
    map: &[Vec<Cells>],
    dirties: &[(usize, usize)],
    robot: (usize, usize),
) -> [[i32; MAX_DIRTY + 1]; MAX_DIRTY + 1] {
    let (width, height) = (map[0].len(), map.len());
    let mut matrix = [[i32::MAX; MAX_DIRTY + 1]; MAX_DIRTY + 1];

    for i in 0..=dirties.len() {
        matrix[i][i] = 0;
    }

    for (idx, &start) in std::iter::once(&robot).chain(dirties).enumerate() {
        let mut visited = vec![vec![false; width]; height];
        visited[start.0][start.1] = true;

        let mut queue = VecDeque::from([(start, 0)]);

        while let Some(((r, c), dist)) = queue.pop_front() {
            let new_dist = dist + 1;
            let adjacents = [
                (r.saturating_sub(1), c),
                (r, c.saturating_sub(1)),
                ((r + 1).min(height - 1), c),
                (r, (c + 1).min(width - 1)),
            ];

            for (adj_r, adj_c) in adjacents {
                if visited[adj_r][adj_c] || matches!(map[adj_r][adj_c], Cells::Wall) {
                    continue;
                }

                visited[adj_r][adj_c] = true;

                if let Cells::Dirty(adj_idx) = map[adj_r][adj_c] {
                    matrix[idx][adj_idx as usize] = new_dist;
                    matrix[adj_idx as usize][idx] = new_dist;
                }

                queue.push_back(((adj_r, adj_c), new_dist));
            }
        }
    }

    matrix
}

fn permutations(
    depth: usize,
    selected: &mut Vec<usize>,
    visited: &mut [bool],
    dist_matrix: &[[i32; MAX_DIRTY + 1]],
    robot: (usize, usize),
    min_dist: &mut i32,
) {
    if depth == selected.len() {
        let mut dist = dist_matrix[ROBOT_IDX][selected[0]];

        for i in 1..selected.len() {
            if dist >= *min_dist {
                return;
            }

            dist = dist.saturating_add(dist_matrix[selected[i - 1]][selected[i]]);
        }

        *min_dist = dist.min(*min_dist);
        return;
    }

    for i in 0..selected.len() {
        if visited[i] {
            continue;
        }

        visited[i] = true;
        selected[depth] = i + 1;

        permutations(depth + 1, selected, visited, dist_matrix, robot, min_dist);

        visited[i] = false;
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
