fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim();
    let n_multiple_16 = format!("{n}0000");
    let n_padded = format!("0000{n}");

    let mut n_multiple_17: Vec<_> = n_multiple_16
        .chars()
        .zip(n_padded.chars())
        .map(|(a, b)| a as u8 + b as u8 - '0' as u8 * 2)
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

    for c in n_multiple_17 {
        print!("{c}");
    }
}
