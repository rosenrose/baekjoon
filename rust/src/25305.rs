fn main() {
    let mut str = String::new();
    read_line(&mut str);

    let k = parse_int_vec(&str)[1];
    read_line(&mut str);

    let mut arr = parse_int_vec(&str);

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
