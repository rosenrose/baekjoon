fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim();
    let n_multiple_16 = format!("{n}0000");
    let n_padded = format!("0000{n}");

    let mut n_multiple_17: Vec<_> = n_multiple_16
        .as_bytes()
        .iter()
        .zip(n_padded.as_bytes())
        .map(|(a, b)| a + b - (b'0' * 2))
        .collect();

    for i in (1..n_multiple_17.len()).rev() {
        if n_multiple_17[i] > 1 {
            n_multiple_17[i - 1] += n_multiple_17[i] / 2;
            n_multiple_17[i] %= 2;
        }
    }

    while n_multiple_17[0] > 1 {
        n_multiple_17.insert(0, n_multiple_17[0] / 2);
        n_multiple_17[1] %= 2;
    }

    for bin in n_multiple_17 {
        print!("{bin}");
    }
}
