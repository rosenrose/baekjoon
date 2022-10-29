fn main() {
    let mut buf = String::new();

    let (number, score) = (1..=5)
        .map(|i| {
            read_line(&mut buf);
            let sum: i32 = buf
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .sum();

            (i, sum)
        })
        .max_by_key(|&(_, score)| score)
        .unwrap();

    println!("{number} {score}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
