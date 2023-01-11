fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [x, y] = parse_int_vec(&buf)[..] else { return };
    let mut days = y;

    for i in 1..x {
        days += match i {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => 28,
            _ => 0,
        };
    }

    let day = match (days - 1) % 7 {
        0 => "MON",
        1 => "TUE",
        2 => "WED",
        3 => "THU",
        4 => "FRI",
        5 => "SAT",
        6 => "SUN",
        _ => "",
    };

    println!("{day}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
