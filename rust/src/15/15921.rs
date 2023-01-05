fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        if buf.trim() == "0" {
            "divide by zero"
        } else {
            "1.00"
        }
    );
}
