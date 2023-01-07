use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout = io::BufWriter::new(stdout.lock());

    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let offset = 'A' as u8;

    for _ in 0..parse_int(input.next().unwrap()) {
        let mut nums = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(parse_int);

        let (a, b, s) = (
            nums.next().unwrap(),
            nums.next().unwrap(),
            input.next().unwrap().as_bytes(),
        );

        let convert_table: Vec<_> = ('A' as u8..='Z' as u8)
            .map(|ch| (((a * (ch - offset) as i32 + b) % 26) as u8 + offset) as char)
            .collect();

        let encrypted: String = s
            .iter()
            .map(|ch| convert_table[(ch - offset) as usize])
            .collect();

        writeln!(stdout, "{encrypted}").unwrap();
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
