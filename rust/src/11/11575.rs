use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout = io::BufWriter::new(stdout.lock());

    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    let offset = 'A' as u8;
    let mut convert_table = [0; 26];

    for _ in 0..parse_int(input()) {
        let mut nums = input().split_ascii_whitespace().map(parse_int);
        let (a, b, s) = (
            nums.next().unwrap(),
            nums.next().unwrap(),
            input().as_bytes(),
        );

        for x in 0..26 {
            convert_table[x as usize] = ((a * x + b) % 26) as u8 + offset;
        }

        let encrypted: String = s
            .iter()
            .map(|&ch| convert_table[(ch - offset) as usize] as char)
            .collect();

        writeln!(stdout, "{encrypted}").unwrap();
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
