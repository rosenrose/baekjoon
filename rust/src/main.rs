fn main() {}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

/* use std::io::{stdout, stdin, BufWriter, Write, BufRead};
let stdin = stdin();
let stdout = stdout();
let mut stdin = stdin.lock();
let mut stdout = BufWriter::new(stdout.lock());

stdin.read_line(&mut buf).unwrap();
writeln!(stdout, "{}").unwrap(); */
