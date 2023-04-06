fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    const HEX_PREFIX: &str = "0x";
    const OCT_PREFIX: &str = "0";

    let dec: i32 = (match buf.trim() {
        x if x.starts_with(HEX_PREFIX) => i32::from_str_radix(&x[HEX_PREFIX.len()..], 16),
        x if x.starts_with(OCT_PREFIX) => i32::from_str_radix(&x[OCT_PREFIX.len()..], 8),
        x => x.parse(),
    })
    .unwrap();

    println!("{dec}");
}
