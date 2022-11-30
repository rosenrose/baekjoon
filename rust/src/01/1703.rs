fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim() == "0" {
            return;
        }

        let mut tokens = buf.split_whitespace().map(parse_int);
        let mut leaves = 1;
        let levels = tokens.next().unwrap();

        for _ in 0..levels {
            leaves *= tokens.next().unwrap();
            leaves -= tokens.next().unwrap();
        }

        println!("{leaves}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
