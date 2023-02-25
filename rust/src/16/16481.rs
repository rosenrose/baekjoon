fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let r = buf
        .split_whitespace()
        .map(|s| (s.parse::<f64>().unwrap()).recip())
        .sum::<f64>()
        .recip();

    println!("{r}");
}
// https://ko.wikipedia.org/wiki/방접원
