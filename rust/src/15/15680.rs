fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        if buf.trim().parse::<i32>().unwrap() == 0 {
            "YONSEI"
        } else {
            "Leading the Way to the Future"
        }
    );
}
