fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: u32 = buf.trim().parse().unwrap();
    let length = n.ilog10() + 1;

    let (mut digits, mut count) = (0, 9);

    for len in 1..=length - 1 {
        digits += count * len;
        count *= 10;
    }

    let start = count / 9;

    digits += (n - start + 1) * length;

    println!("{digits}");
}
