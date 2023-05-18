fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut num = buf.trim().to_owned();
    let mut step = 0;

    while num.len() > 1 {
        num = num
            .as_bytes()
            .iter()
            .map(|ch| (ch - b'0') as i32)
            .product::<i32>()
            .to_string();

        step += 1;
    }

    println!("{step}");
}
