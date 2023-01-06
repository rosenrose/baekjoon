fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let init = buf.as_bytes()[0];
    let (count, _) = "ILOVEYONSEI"
        .as_bytes()
        .iter()
        .fold((0, init), |(c, ch), &next| (c + ch.abs_diff(next), next));

    println!("{count}");
}
