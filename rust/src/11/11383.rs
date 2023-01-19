use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (n, m) = (parse_int(input()), parse_int(input()));
    let first: Vec<String> = (0..n)
        .map(|_| input().chars().map(|c| c.to_string().repeat(2)).collect())
        .collect();

    println!(
        "{}",
        if (0..m).zip(first).all(|(_, first)| first == input()) {
            "Eyfa"
        } else {
            "Not Eyfa"
        }
    );
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
