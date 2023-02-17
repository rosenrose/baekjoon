fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let type_ = input.next().unwrap();

    for var in input.map(|s| s.trim_end_matches([',', ';'])) {
        let (name, extra) = if let Some(i) = var.find(['[', ']', '&', '*']) {
            var.split_at(i)
        } else {
            (var, "")
        };

        let extra = extra.chars().rev().collect::<String>().replace("][", "[]");

        println!("{type_}{extra} {name};");
    }
}
