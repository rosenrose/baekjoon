fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let hidden_nums = buf
        .trim()
        .split(char::is_alphabetic)
        .filter(|c| !c.is_empty())
        .map(|s| s.parse::<i64>().unwrap());

    println!("{}", hidden_nums.sum::<i64>());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
