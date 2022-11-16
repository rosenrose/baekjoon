fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [n, r, c] = parse_int_vec(&buf)[..] {
        let size = 2_u32.pow(n);

        println!("{}", visit_z(r, c, 0, 0, size, 0));
    }
}

fn visit_z(r: u32, c: u32, x: u32, y: u32, n: u32, start: u32) -> u32 {
    if r == y && c == x {
        return start;
    }

    let diff = n * n;

    match (r < y + n / 2, c < x + n / 2) {
        (true, true) => visit_z(r, c, x, y, n / 2, start),
        (true, false) => visit_z(r, c, x + n / 2, y, n / 2, start + diff / 4),
        (false, true) => visit_z(r, c, x, y + n / 2, n / 2, start + diff / 4 * 2),
        (false, false) => visit_z(r, c, x + n / 2, y + n / 2, n / 2, start + diff / 4 * 3),
    }
}

fn parse_int_vec(buf: &String) -> Vec<u32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
