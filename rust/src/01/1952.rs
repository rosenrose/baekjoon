fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [r, c] = parse_int_vec(&buf)[..] else { return };
    let mut is_visit = vec![vec![false; c as usize]; r as usize];
    let (mut x, mut y) = (-1, 0);
    let mut direction = (1, 0);
    let mut count = 0;

    loop {
        loop {
            let (next_x, next_y) = (x + direction.0, y + direction.1);

            if (next_x == -1 || next_x == c || next_y == -1 || next_y == r)
                || is_visit[next_y as usize][next_x as usize]
            {
                break;
            }

            (x, y) = (next_x, next_y);
            is_visit[y as usize][x as usize] = true;
        }

        let adjacents = [
            ((x - 1).max(0), y),
            (x, (y - 1).max(0)),
            ((x + 1).min(c - 1), y),
            (x, (y + 1).min(r - 1)),
        ];

        if adjacents
            .iter()
            .all(|&(adj_x, adj_y)| is_visit[adj_y as usize][adj_x as usize])
        {
            break;
        }

        direction = match direction {
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            _ => Default::default(),
        };
        count += 1;
    }

    println!("{count}");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
