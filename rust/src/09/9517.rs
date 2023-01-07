use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (mut k, n) = (parse_int(input()), parse_int(input()));
    let mut time = 3 * 60 + 30;
    const M: i32 = 8;

    for (t, z) in (0..n).map(|_| (parse_int(input()), input())) {
        time -= t;

        if time <= 0 {
            break;
        }

        if z == "T" {
            k = (k + 1) % M;
        }
    }

    println!("{}", if k == 0 { M } else { k });
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
