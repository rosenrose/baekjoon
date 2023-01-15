fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    [1, 1, 2, 2, 2, 8]
        .iter()
        .zip(buf.split_whitespace().flat_map(str::parse::<i32>))
        .for_each(|(x, y)| {
            print!("{} ", x - y);
        });
}
