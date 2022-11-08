fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let offset = 'A' as u8;

    for _ in 0..n {
        read_line(&mut buf);

        if let [a, b] = parse_int_vec(&buf)[..] {
            read_line(&mut buf);

            let convert_table: Vec<_> = ('A'..='Z')
                .map(|c| (((a * (c as u8 - offset) as i32 + b) % 26) as u8 + offset) as char)
                .collect();

            let encrypted: String = buf
                .trim()
                .chars()
                .map(|c| convert_table[(c as u8 - offset) as usize])
                .collect();

            println!("{encrypted}");
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
