fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);
        let mut input = buf.trim().chars();

        let first = input.next().unwrap();

        let last = match input.next_back() {
            Some(c) => c,
            None => first,
        };

        println!("{first}{last}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
