fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let len = buf.trim().len();

    let bin = if len % 3 == 0 {
        buf.trim().to_string()
    } else {
        let pad = 3 - (len % 3);
        format!("{}{}", "0".repeat(pad), buf.trim())
    };

    let bin: Vec<String> = bin.chars().map(|c| c.to_string()).collect();

    let oct: String = bin
        .chunks(3)
        .map(|s| {
            let digit = u32::from_str_radix(&s.concat(), 2).unwrap();
            char::from_digit(digit, 8).unwrap()
        })
        .collect();

    println!("{oct}");
}
