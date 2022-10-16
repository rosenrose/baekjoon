fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut arr: Vec<i32> = buf
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    arr.sort_by(|a, b| b.cmp(a));

    for c in arr {
        print!("{c}");
    }
}
