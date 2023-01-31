fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [d, m] = parse_int_vec(&buf)[..] else { return };
    let mut days = d;

    for i in 1..m {
        days += match i {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => 28,
            _ => Default::default(),
        };
    }

    let day = match (days - 1) % 7 {
        0 => "Thursday",
        1 => "Friday",
        2 => "Saturday",
        3 => "Sunday",
        4 => "Monday",
        5 => "Tuesday",
        6 => "Wednesday",
        _ => Default::default(),
    };

    println!("{day}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
