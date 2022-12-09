fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace();
    let n = input.next().unwrap();
    let b: u32 = input.next().unwrap().parse().unwrap();

    println!("{}", i32::from_str_radix(n, b).unwrap());
}
