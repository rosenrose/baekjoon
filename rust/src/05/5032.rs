fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [mut empty, found, exchange] = parse_int_vec(&buf)[..] else { return };
    empty += found;

    let mut count = 0;

    while empty >= exchange {
        let new = empty / exchange;

        count += new;
        empty = (empty % exchange) + new;
    }

    println!("{count}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
