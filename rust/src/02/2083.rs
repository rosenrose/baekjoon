fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);
        let name = buf.split_whitespace().next().unwrap();

        if name == "#" {
            return;
        }

        if let [age, weight] = parse_int_vec(&buf[name.len()..])[..] {
            let class = if age > 17 || weight >= 80 {
                "Senior"
            } else {
                "Junior"
            };

            println!("{name} {class}");
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
