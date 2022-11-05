fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!("{}", i32::from_str_radix(buf.trim(), 16).unwrap());
}
