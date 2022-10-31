fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let bin: Vec<String> = buf.trim().chars().map(|c| c.to_string()).collect();

    let oct: String = bin
        .rchunks(3)
        .rev()
        .map(|s| {
            let digit = u32::from_str_radix(&s.concat(), 2).unwrap();
            char::from_digit(digit, 8).unwrap()
        })
        .collect();

    println!("{oct}");
}
