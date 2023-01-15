use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [mut left, mut right, a] = parse_int_vec(&buf)[..] else { return };
    let diff = left.abs_diff(right);
    let both = a.saturating_sub(diff);

    match left.cmp(&right) {
        Ordering::Less => left += diff.min(a),
        Ordering::Greater => right += diff.min(a),
        _ => (),
    }

    if left == right {
        left += both / 2;
        right += both / 2;
    }

    println!("{}", left.min(right) * 2);
}

fn parse_int_vec(buf: &String) -> Vec<u32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
