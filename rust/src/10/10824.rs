fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c, d] = buf.split_whitespace().collect::<Vec<_>>()[..] else { return };
    let ab: i64 = [a, b].concat().parse().unwrap();
    let cd: i64 = [c, d].concat().parse().unwrap();

    println!("{}", ab + cd);
}
