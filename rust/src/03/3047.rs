fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let mut abc = parse_int_vec(&buf);
    abc.sort();

    read_line(&mut buf);

    for ch in buf.trim().chars() {
        print!("{} ", abc[(ch as u8 - 'A' as u8) as usize]);
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
