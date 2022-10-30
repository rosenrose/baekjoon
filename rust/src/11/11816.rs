fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let dec: i32 = match buf.trim() {
        x if &x[..2] == "0x" => i32::from_str_radix(&x[2..], 16),
        x if &x[..1] == "0" => i32::from_str_radix(&x[1..], 8),
        _ => buf.trim().parse(),
    }
    .unwrap();

    println!("{dec}");
}
