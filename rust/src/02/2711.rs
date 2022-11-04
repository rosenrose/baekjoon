fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(buf.trim());

    for _ in 0..n {
        read_line(&mut buf);

        if let [index, typo] = parse_str_vec(&buf)[..] {
            let index = parse_int(index) - 1;
            let mut typo = typo.to_string();

            typo.remove(index);

            println!("{typo}");
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
