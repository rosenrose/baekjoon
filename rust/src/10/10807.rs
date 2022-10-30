fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let arr = parse_int_vec(&buf);
    read_line(&mut buf);

    let v: i32 = buf.trim().parse().unwrap();
    let count = arr.iter().filter(|&&n| n == v).count();

    println!("{count}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
