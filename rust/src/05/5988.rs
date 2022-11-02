fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        let last = buf.trim().chars().last().unwrap().to_digit(10).unwrap();

        println!("{}", if last % 2 == 0 { "even" } else { "odd" });
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
