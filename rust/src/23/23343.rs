fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [x, y] = parse_int_vec(&buf)[..] else {
        println!("NaN");
        return;
    };

    println!("{}", x - y);
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
