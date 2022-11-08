fn main() {
    let mut buf = String::new();

    let mut scores: Vec<_> = (1..=8)
        .map(|i| {
            read_line(&mut buf);
            let score = parse_int(&buf);

            (score, i)
        })
        .collect();
    scores.sort_by_key(|&(s, _)| s);

    let (third, _) = scores[2];

    let mut top_5: Vec<_> = scores.iter().filter(|&&(s, _)| s > third).collect();
    top_5.sort_by_key(|(_, i)| i);

    let sum: i32 = top_5.iter().map(|(s, _)| s).sum();

    println!("{sum}");
    for (_, i) in top_5 {
        print!("{i} ");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
