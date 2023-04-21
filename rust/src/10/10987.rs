fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let vowels = buf.trim().matches(['a', 'e', 'i', 'o', 'u']).count();

    println!("{vowels}");
}
