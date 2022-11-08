fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);
        let mut scores = parse_int_vec(&buf);

        scores.sort();
        scores.pop();
        scores.remove(0);

        if scores[0].abs_diff(*scores.last().unwrap()) >= 4 {
            println!("KIN");
            continue;
        }

        println!("{}", scores.iter().sum::<i32>());
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
