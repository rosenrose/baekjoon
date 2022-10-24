fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let k = parse_int_vec(&buf)[1];
    read_line(&mut buf);

    let mut arr = parse_int_vec(&buf);

    arr.sort_by(|a, b| b.cmp(a));

    println!("{}", arr[(k - 1) as usize]);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
