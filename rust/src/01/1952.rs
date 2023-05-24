fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [height, width] = parse_int_vec(&buf)[..] else { return };
    let mut is_visit = vec![vec![false; width as usize]; height as usize];

    let (mut r, mut c) = (0, -1);
    let mut dir = (0, 1);
    let mut count = 0;

    loop {
        loop {
            let (next_r, next_c) = (r + dir.0, c + dir.1);

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

        dir = match dir {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => unreachable!(),
        };
        count += 1;
    }

    println!("{count}");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
