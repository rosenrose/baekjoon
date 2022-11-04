fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    const R: i64 = 31;
    const M: i64 = 1234567891;

    let hash = buf
        .trim()
        .char_indices()
        .map(|(i, c)| {
            let num = (c as u8 - 'a' as u8 + 1) as i64;
            let mut remainder = num % M;

            for _ in 0..i {
                remainder = ((remainder % M) * (R % M)) % M;
            }

            remainder
        })
        .sum::<i64>()
        % M;

    println!("{hash}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
