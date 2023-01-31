fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let ratio = parse_int_vec(&buf);

    if ratio.iter().sum::<i32>() >= 100 {
        println!("OK");
        return;
    }

    let least = match ratio.iter().enumerate().min_by_key(|(_, &r)| r).unwrap().0 {
        0 => "Soongsil",
        1 => "Korea",
        2 => "Hanyang",
        _ => Default::default(),
    };

    println!("{least}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
