fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let scores = parse_int_vec(&buf);
    let min = scores.iter().min().unwrap();
    let max = scores.iter().max().unwrap();

    // let (mut min, mut max) = (1000, 0);
    // buf.split_whitespace().for_each(|s| {
    //     let score: i32 = s.parse().unwrap();

    //     if score < min {
    //         min = score;
    //     }
    //     if score > max {
    //         max = score;
    //     }
    // });

    println!("{}", max - min);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
