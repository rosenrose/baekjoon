fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    read_line(&mut buf);

    let pattern = buf.trim().to_string();
    let mut tokens = pattern.split('*');
    let (prefix, suffix) = (tokens.next().unwrap(), tokens.next().unwrap());

    for _ in 0..n {
        read_line(&mut buf);
        let file_name = buf.trim();

        if file_name.len() < prefix.len() + suffix.len() {
            println!("NE");
            continue;
        }

        if file_name.starts_with(prefix) && file_name.ends_with(suffix) {
            println!("DA");
        } else {
            println!("NE");
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
