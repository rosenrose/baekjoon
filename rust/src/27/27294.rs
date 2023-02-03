fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        if matches!(parse_int_vec(&buf)[..], [12..=16, 0]) {
            320
        } else {
            280
        }
    );
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
