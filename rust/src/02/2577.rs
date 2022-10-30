fn main() {
    let mut buf = String::new();

    let product = (0..3)
        .map(|_| {
            read_line(&mut buf);
            buf.trim().parse::<i32>().unwrap()
        })
        .product::<i32>()
        .to_string();

    for digit in '0'..='9' {
        let count = product.chars().filter(|&c| c == digit).count();

        println!("{count}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
