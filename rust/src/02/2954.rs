fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.trim().to_owned();
    let patterns = ["a", "e", "i", "o", "u"]
        .iter()
        .map(|vowel| (vowel, format!("{vowel}p{vowel}")));

    for (v, p) in patterns {
        input = input.replace(&p, v);
    }

    println!("{input}");
}
