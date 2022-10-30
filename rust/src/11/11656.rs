fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let word = buf.trim();

    let mut prefixes: Vec<&str> = (0..word.len()).map(|i| &word[i..]).collect();

    prefixes.sort();

    println!("{}", prefixes.join("\n"));
}
