fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let length = buf.trim().len() as i64;
    let n: i64 = buf.trim().parse().unwrap();

    let (mut digits, mut count) = (0, 9);

    for len in 1..=length - 1 {
        digits += count * len;
        count *= 10;
    }

    let start = count / 9;

    digits += (n - start + 1) * length;

    println!("{digits}");
}
