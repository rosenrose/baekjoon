fn main() {
    let mut buf = String::new();

    const N: usize = 3;
    let mut x_points = [0; N];
    let mut y_points = [0; N];

    for i in 0..N {
        read_line(&mut buf);

        if let [x, y] = parse_int_vec(&buf)[..] {
            (x_points[i], y_points[i]) = (x, y);
        }
    }

    let get_unique = |points: [i32; 3]| match points {
        [a, b, c] if a == b => Some(c),
        [a, b, c] if a == c => Some(b),
        [a, b, c] if b == c => Some(a),
        _ => None,
    };

    let (x, y) = (get_unique(x_points).unwrap(), get_unique(y_points).unwrap());

    println!("{x} {y}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
