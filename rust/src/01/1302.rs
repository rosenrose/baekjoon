fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let titles = parse_str_vec_lines(&mut buf, n);

    let mut max_count = 0;
    let counts: Vec<(&String, usize)> = titles
        .iter()
        .map(|title| {
            let count = titles.iter().filter(|&t| t == title).count();
            if count > max_count {
                max_count = count;
            }

            (title, count)
        })
        .collect();

    let mut best_sellers: Vec<&String> = counts
        .iter()
        .filter(|(_, count)| *count == max_count)
        .map(|&(title, _)| title)
        .collect();

    best_sellers.sort();

    println!("{}", best_sellers[0]);
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
