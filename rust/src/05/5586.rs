fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim().as_bytes();
    let counts: Vec<_> = ["JOI", "IOI"]
        .iter()
        .map(|s| input.windows(3).filter(|&w| w == s.as_bytes()).count())
        .collect();

    println!("{}\n{}", counts[0], counts[1]);
}
