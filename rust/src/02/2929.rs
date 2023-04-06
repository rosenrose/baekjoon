fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let tokens: Vec<_> = buf.trim().split(char::is_uppercase).skip(1).collect();
    let nop_count = tokens[..tokens.len() - 1]
        .iter()
        .fold(0, |acc, params| acc + 3 - (params.len() % 4));

    println!("{nop_count}");
}
