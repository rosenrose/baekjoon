fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let r_inverse: f32 = buf
        .split_whitespace()
        .map(|s| 1.0 / s.parse::<f32>().unwrap())
        .sum();

    println!("{}", 1.0 / r_inverse);
}
// https://ko.wikipedia.org/wiki/방접원
