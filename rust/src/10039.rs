fn main() {
    const N: i32 = 5;
    let mut buf = String::new();

    let scores = (0..N).map(|_| {
        read_line(&mut buf);
        let score = parse_int(&buf);

        if score < 40 {
            40
        } else {
            score
        }
    });

    println!("{}", scores.sum::<i32>() / N);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
