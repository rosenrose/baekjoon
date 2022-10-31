use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, BufRead, BufWriter, Write};
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    stdin().read_to_string(&mut buf).unwrap();

    stdin.read_line(&mut buf).unwrap();
    writeln!(stdout, "").unwrap();
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

fn parse_int_vec_lines(buf: &mut String, n: i32) -> Vec<i32> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf)
        })
        .collect()
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}

fn parse_str_vec_lines(buf: &mut String, n: i32) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}

fn parse_int_set(buf: &String) -> HashSet<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_int_set_lines(buf: &mut String, n: i32) -> HashSet<i32> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf)
        })
        .collect()
}

fn parse_str_set_lines(buf: &mut String, n: i32) -> HashSet<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}

fn parse_matrix(buf: &mut String, rows: i32) -> Vec<Vec<i32>> {
    (0..rows)
        .map(|_| {
            read_line(buf);
            parse_int_vec(buf)
        })
        .collect()
}

use std::string::ToString;

fn vec_join<T>(vec: &Vec<T>, seperator: &str) -> String
where
    T: ToString,
{
    vec.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(seperator)
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
