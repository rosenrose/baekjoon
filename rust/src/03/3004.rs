fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let horizontal_lines = n / 2;
    let vertical_lines = n - horizontal_lines;

    println!("{}", (horizontal_lines + 1) * (vertical_lines + 1));
}
