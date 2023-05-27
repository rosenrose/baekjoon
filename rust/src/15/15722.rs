const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut t: i32 = buf.trim().parse().unwrap();
    let (mut x, mut y) = (0, 0);
    let mut dir = 0;
    let mut len = 1;

    loop {
        for _ in 0..len {
            if t == 0 {
                println!("{x} {y}");
                return;
            }

            (x, y) = (x + DIRS[dir].0, y + DIRS[dir].1);
            t -= 1;
        }

        if dir & 1 == 1 {
            len += 1;
        }

        dir = (dir + 1) % DIRS.len();
    }
}
