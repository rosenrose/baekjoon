fn main() {
    let mut buf = String::new();
}

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

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/* use std::io::{stdout, stdin, BufWriter, Write, BufRead};
let stdin = stdin();
let stdout = stdout();
let mut stdin = stdin.lock();
let mut stdout = BufWriter::new(stdout.lock());

stdin.read_line(&mut buf).unwrap();
writeln!(stdout, "{}").unwrap(); */
