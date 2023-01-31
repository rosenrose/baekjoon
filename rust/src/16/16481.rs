fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let r = buf
        .split_whitespace()
        .map(|s| (s.parse::<f32>().unwrap()).recip())
        .sum::<f32>()
        .recip();

    println!("{r}");
}
// https://ko.wikipedia.org/wiki/방접원
