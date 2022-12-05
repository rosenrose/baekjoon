use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = parse_int(input.next().unwrap());
    let offset = 'A' as u8;

    for _ in 0..n {
        let (a, b, s) = (
            parse_int(input.next().unwrap()),
            parse_int(input.next().unwrap()),
            input.next().unwrap(),
        );

        let convert_table: Vec<_> = (0..26)
            .map(|x| (((a * x + b) % 26) as u8 + offset) as char)
            .collect();

        let encrypted: String = s
            .as_bytes()
            .iter()
            .map(|c| convert_table[(c - offset) as usize])
            .collect();

        writeln!(stdout, "{encrypted}").unwrap();
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
