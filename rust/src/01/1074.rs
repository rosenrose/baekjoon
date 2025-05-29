use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let [n, r, c] = [(); 3].map(|_| input.next().unwrap());
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
