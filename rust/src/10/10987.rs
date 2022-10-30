fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let vowels = buf
        .trim()
        .chars()
        .filter(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count();

    println!("{vowels}");
}
