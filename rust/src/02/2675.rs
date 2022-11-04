fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        if let [r, s] = parse_str_vec(&buf)[..] {
            let r: usize = r.parse().unwrap();
            let p: String = s.chars().map(|c| c.to_string().repeat(r)).collect();

            println!("{p}");
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
