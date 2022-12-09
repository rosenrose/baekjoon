fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let a = i128::from_str_radix(input.next().unwrap(), 2).unwrap();
    let b = i128::from_str_radix(input.next().unwrap(), 2).unwrap();

    println!("{:b}", a + b);
}
