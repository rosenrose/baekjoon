fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let (n, b) = buf.trim().split_once(' ').unwrap();
    let b: u32 = b.parse().unwrap();

    println!("{}", i32::from_str_radix(n, b).unwrap());
}
