fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let parse_int = |s: &str| s.parse::<i32>().unwrap();
    let n = parse_int(buf.trim());

    let infos: Vec<_> = (0..n)
        .map(|_| {
            read_line(&mut buf);
            let mut tokens = buf.split_whitespace();
            let name = tokens.next().unwrap().to_string();

            match tokens.map(parse_int).collect::<Vec<_>>()[..] {
                [d, m, y] => (name, (y, m, d)),
                _ => Default::default(),
            }
        })
        .collect();

    let (youngest, _) = infos.iter().max_by_key(|(_, birth)| birth).unwrap();
    let (oldest, _) = infos.iter().min_by_key(|(_, birth)| birth).unwrap();

    println!("{youngest}\n{oldest}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
