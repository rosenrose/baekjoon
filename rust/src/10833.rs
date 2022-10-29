fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let mut extra_apple = 0;

    for _ in 0..n {
        read_line(&mut buf);

        if let [student, apple] = parse_int_vec(&buf)[..] {
            extra_apple += apple % student;
        }
    }

    println!("{extra_apple}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
