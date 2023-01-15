fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, d] = parse_float_vec(&buf)[..] else { return };

    println!("{:.1}", n / 2.0 - d);
}
// https://wiki.mathnt.net/index.php?title=나비정리
fn parse_float_vec(buf: &String) -> Vec<f32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
