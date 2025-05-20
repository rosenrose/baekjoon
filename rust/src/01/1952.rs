use std::io;

const WIDTH_MAX: usize = 100;
const HEIGHT_MAX: usize = 100;
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_whitespace().flat_map(str::parse::<i32>);

    let [height, width] = [(); 2].map(|_| input.next().unwrap());
    let mut is_visit = [[false; WIDTH_MAX]; HEIGHT_MAX];

    let (mut r, mut c) = (0, -1);
    let mut dir = 0;
    let mut count = 0;

    loop {
        loop {
            let (next_r, next_c) = (r + DIRS[dir].0, c + DIRS[dir].1);

            if (next_r == -1 || next_r == height || next_c == -1 || next_c == width)
                || is_visit[next_r as usize][next_c as usize]
            {
                break;
            }

            (r, c) = (next_r, next_c);
            is_visit[r as usize][c as usize] = true;
        }

        let adjacents = [
            ((r - 1).max(0), c),
            (r, (c - 1).max(0)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        if adjacents
            .iter()
            .all(|&(adj_r, adj_c)| is_visit[adj_r as usize][adj_c as usize])
        {
            break;
        }

        dir = (dir + 1) % DIRS.len();
        count += 1;
    }

    println!("{count}");
}
