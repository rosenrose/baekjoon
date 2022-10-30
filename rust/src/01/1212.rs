fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let oct = buf.trim();

    let bin: Vec<String> = oct
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let digit = c.to_digit(10).unwrap();

            if i == 0 {
                format!("{digit:b}")
            } else {
                format!("{digit:03b}")
            }
        })
        .collect();

    println!("{}", bin.concat());
}
