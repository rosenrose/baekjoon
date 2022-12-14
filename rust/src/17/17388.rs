fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let ratio = parse_int_vec(&buf);

    if ratio.iter().sum::<i32>() >= 100 {
        println!("OK");
        return;
    }

    let least = match ratio.iter().enumerate().min_by_key(|(_, &r)| r).unwrap() {
        (0, _) => "Soongsil",
        (1, _) => "Korea",
        (2, _) => "Hanyang",
        _ => "",
    };

    println!("{least}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
