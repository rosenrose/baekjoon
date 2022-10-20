fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [x, y] = parse_int_vec(&buf)[..] {
        let mut days = y;

        for i in 1..x {
            match i {
                i if [1, 3, 5, 7, 8, 10, 12].contains(&i) => days += 31,
                i if [4, 6, 9, 11].contains(&i) => days += 30,
                2 => days += 28,
                _ => (),
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
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
