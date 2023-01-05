fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut num = buf.trim().to_string();
    let mut step = 0;

    while num.len() > 1 {
        num = num
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .product::<u32>()
            .to_string();

        step += 1;
    }

    println!("{step}");
}
