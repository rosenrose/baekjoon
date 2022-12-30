fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    for i in 1..=buf.trim().parse().unwrap() {
        println!("Material Management {i}");
        println!("Classification ---- End!");
    }
}
