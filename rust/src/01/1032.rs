fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let file_names = parse_str_vec_lines(&mut buf, n);

    if file_names.len() == 1 {
        println!("{}", file_names[0]);
        return;
    }

    let pattern: Vec<String> = (0..file_names[0].len())
        .map(|i| {
            let letter = file_names[0].chars().nth(i);

            let is_same = file_names[1..]
                .iter()
                .map(|file_name| file_name.chars().nth(i))
                .all(|c| c == letter);

            (if is_same { letter.unwrap() } else { '?' }).to_string()
        })
        .collect();

    println!("{}", pattern.concat());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec_lines(buf: &mut String, n: i32) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
