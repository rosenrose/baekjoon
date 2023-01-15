fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [r_out, r_in] = parse_float_vec(&buf)[..] else { return };

    println!("{}", r_out * (r_out - 2.0 * r_in).floor());
}
// https://ko.wikipedia.org/wiki/오일러_삼각형_정리
fn parse_float_vec(buf: &String) -> Vec<f64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
