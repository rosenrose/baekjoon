fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    const D2: &str = "D2";

    println!(
        "{}",
        if buf.to_uppercase().contains(D2) {
            D2
        } else {
            "unrated"
        }
    );
}
