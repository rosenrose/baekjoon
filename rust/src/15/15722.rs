fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut t: i32 = buf.trim().parse().unwrap();
    let (mut x, mut y) = (0, 0);
    let mut direction = (0, 1);
    let mut len = 1;

    loop {
        for _ in 0..len {
            if t == 0 {
                println!("{x} {y}");
                return;
            }

            (x, y) = (x + direction.0, y + direction.1);
            t -= 1;
        }

        direction = match direction {
            (0, 1) => (1, 0),
            (1, 0) => {
                len += 1;
                (0, -1)
            }
            (0, -1) => (-1, 0),
            (-1, 0) => {
                len += 1;
                (0, 1)
            }
            _ => Default::default(),
        };
    }
}
