fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let (q, r) = (n / 4, n % 4);

    #[rustfmt::skip]
    let index = match r {
        0 | 2 => if q % 2 == 0 { 2 } else { 4 },
        1     => if q % 2 == 0 { 1 } else { 5 },
        3 => 3,
        _ => unreachable!(),
    };

    println!("{index}");
}
