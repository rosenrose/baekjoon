use std::collections::VecDeque;
use std::io;

#[derive(Copy, Clone)]
enum Cells {
    Clean,
    Dirty(u8),
    Wall,
}

const ROBOT_IDX: usize = 0;
const WIDTH_MAX: usize = 20;
const HEIGHT_MAX: usize = 20;
const DIRTY_MAX: usize = 10;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    while let (width @ 1.., height @ 1..) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    ) {
        let mut dirties = [(0, 0); DIRTY_MAX];
        let mut dirties_len = 0;
        let mut robot = (0, 0);

        let mut map = [[Cells::Clean; WIDTH_MAX]; HEIGHT_MAX];

        for (r, row) in input.by_ref().take(height).enumerate() {
            for (c, ch) in row.char_indices() {
                map[r][c] = match ch {
                    '.' => Cells::Clean,
                    '*' => {
                        dirties[dirties_len] = (r, c);
                        dirties_len += 1;
                        Cells::Dirty(dirties_len as u8)
                    }
                    'x' => Cells::Wall,
                    'o' => {
                        robot = (r, c);
                        Cells::Clean
                    }
                    _ => unreachable!(),
                };
            }
        }

        println!(
            "{}",
            simulate(&map[..height], width, &dirties[..dirties_len], robot).unwrap_or(-1)
        );
    }
}

fn simulate(
    map: &[[Cells; WIDTH_MAX]],
    width: usize,
    dirties: &[(usize, usize)],
    robot: (usize, usize),
) -> Option<i32> {
    let dist_matrix = get_dists(map, width, dirties, robot);

    if dist_matrix[..dirties.len()]
        .iter()
        .flat_map(|row| &row[..dirties.len()])
        .any(|&dist| dist == i32::MAX)
    {
        return None;
    }

    let mut min_cost = i32::MAX;

    permutations(
        0,
        &mut [0; DIRTY_MAX][..dirties.len()],
        &mut [false; DIRTY_MAX + 1],
        &dist_matrix,
        robot,
        0,
        &mut min_cost,
    );

    (min_cost != i32::MAX).then_some(min_cost)
}

fn get_dists(
    map: &[[Cells; WIDTH_MAX]],
    width: usize,
    dirties: &[(usize, usize)],
    robot: (usize, usize),
) -> [[i32; DIRTY_MAX + 1]; DIRTY_MAX + 1] {
    let height = map.len();
    let mut matrix = [[i32::MAX; DIRTY_MAX + 1]; DIRTY_MAX + 1];

    for i in 0..=dirties.len() {
        matrix[i][i] = 0;
    }

    for (idx, &start) in std::iter::once(&robot).chain(dirties).enumerate() {
        let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
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
    selected: &mut [usize],
    visited: &mut [bool],
    dist_matrix: &[[i32; DIRTY_MAX + 1]],
    robot: (usize, usize),
    cost: i32,
    min_cost: &mut i32,
) {
    if depth == selected.len() {
        *min_cost = cost.min(*min_cost);
        return;
    }

    for i in 1..=selected.len() {
        if visited[i] {
            continue;
        }

        selected[depth] = i;
        let new_cost = cost.saturating_add(if depth == 0 {
            dist_matrix[ROBOT_IDX][selected[0]]
        } else {
            dist_matrix[selected[depth - 1]][selected[depth]]
        });

        if new_cost >= *min_cost {
            continue;
        }

        visited[i] = true;

        permutations(
            depth + 1,
            selected,
            visited,
            dist_matrix,
            robot,
            new_cost,
            min_cost,
        );

        visited[i] = false;
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
