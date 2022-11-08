fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let parse_int = |s: &str| s.parse::<i32>().unwrap();
    let n = parse_int(buf.trim());

    for i in 1..=n {
        read_line(&mut buf);

        let mut scores: Vec<_> = buf.split_whitespace().skip(1).map(parse_int).collect();
        scores.sort();

        let min = scores[0];
        let max = scores.last().unwrap();
        let largest_gap = (0..scores.len() - 1)
            .map(|j| scores[j].abs_diff(scores[j + 1]))
            .max()
            .unwrap();

        println!("Class {i}\nMax {max}, Min {min}, Largest gap {largest_gap}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
