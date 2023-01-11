fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c] = parse_float_vec(&buf)[..] else { return };
    let s = (a + b + c) / 2.0;
    let area_sq = s * (s - a) * (s - b) * (s - c);
    let area = area_sq.sqrt();

    let r_out_sq = a * a * b * b * c * c / (area_sq * 16.0);
    let r_in_sq = area_sq / (s * s);
    let (r_out, r_in) = (r_out_sq.sqrt(), r_in_sq.sqrt());

    let d = if r_out >= 2.0 * r_in {
        (r_out * (r_out - 2.0 * r_in)).sqrt()
    } else {
        0.0
    };
    let k = r_out + r_in;

    println!("{area}\n{r_out}\n{r_in}\n{d}\n{k}");
}
// https://ko.wikipedia.org/wiki/외접원
// https://ko.wikipedia.org/wiki/카르노의_정리
fn parse_float_vec(buf: &String) -> Vec<f64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
