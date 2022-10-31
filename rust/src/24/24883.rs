fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    match buf.trim() {
        "N" | "n" => println!("Naver D2"),
        _ => println!("Naver Whale"),
    };
}
