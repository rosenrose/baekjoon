fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace();
    let n = input.next().unwrap();
    let m = input.next().unwrap();

    println!("{}", if n == m { 1 } else { 0 });
}
