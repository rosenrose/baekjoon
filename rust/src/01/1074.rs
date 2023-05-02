fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, r, c] = parse_int_vec(&buf)[..] else { return };
    let size = 1 << n;

    println!("{}", visit_z(r, c, 0, 0, size, 0));
}

fn visit_z(r: u32, c: u32, x: u32, y: u32, n: u32, start: u32) -> u32 {
    if r == y && c == x {
        return start;
    }

    let diff = n * n;
    let half = n / 2;

    match (r < y + half, c < x + half) {
        (true, true) => visit_z(r, c, x, y, half, start),
        (true, false) => visit_z(r, c, x + half, y, half, start + diff / 4),
        (false, true) => visit_z(r, c, x, y + half, half, start + diff / 4 * 2),
        (false, false) => visit_z(r, c, x + half, y + half, half, start + diff / 4 * 3),
    }
}

fn parse_int_vec(buf: &str) -> Vec<u32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
