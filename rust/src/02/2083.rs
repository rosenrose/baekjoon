fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);
        let mut tokens = buf.split_whitespace();
        let name = tokens.next().unwrap();

        if name == "#" {
            return;
        }

        if let [age, weight] = tokens
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()[..]
        {
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
