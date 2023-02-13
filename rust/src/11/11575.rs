use std::io::{self, Write};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let stdout = io::stdout();
    let mut stdout = io::BufWriter::new(stdout.lock());

    let n = parse_int(input());
    let offset = 'A' as u8;
    let mut convert_table = [0; 26];

    for (a, b, s) in (0..n).map(|_| (parse_int(input()), parse_int(input()), input())) {
        for x in 0..26 {
            convert_table[x as usize] = ((a * x + b) % 26) as u8 + offset;
        }

        let encrypted: String = s
            .chars()
            .map(|ch| convert_table[ch as usize - offset as usize] as char)
            .collect();

        writeln!(stdout, "{encrypted}").unwrap();
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
