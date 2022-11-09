use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let n: i32 = lines.next().unwrap().parse().unwrap();
    let offset = 'A' as u8;

    for _ in 0..n {
        if let [a, b] = parse_int_vec(lines.next().unwrap())[..] {
            let convert_table: Vec<_> = ('A'..='Z')
                .map(|c| (((a * (c as u8 - offset) as i32 + b) % 26) as u8 + offset) as char)
                .collect();

            let encrypted: String = lines
                .next()
                .unwrap()
                .chars()
                .map(|c| convert_table[(c as u8 - offset) as usize])
                .collect();

            writeln!(stdout, "{encrypted}").unwrap();
        }
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
