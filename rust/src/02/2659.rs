fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut clock_nums = Vec::new();

    for a in 1..=9 {
        for b in 1..=9 {
            for c in 1..=9 {
                for d in 1..=9 {
                    let clock_num = [
                        format!("{a}{b}{c}{d}"),
                        format!("{b}{c}{d}{a}"),
                        format!("{c}{d}{a}{b}"),
                        format!("{d}{a}{b}{c}"),
                    ]
                    .iter()
                    .map(|s| s.parse::<i32>().unwrap())
                    .min()
                    .unwrap();

                    clock_nums.push(clock_num);
                }
            }
        }
    }

    clock_nums.sort();
    clock_nums.dedup();
    // println!("{clock_nums:?} {}", clock_nums.len());
    if let [a, b, c, d] = parse_int_vec(&buf)[..] {
        let clock_num = [
            format!("{a}{b}{c}{d}"),
            format!("{b}{c}{d}{a}"),
            format!("{c}{d}{a}{b}"),
            format!("{d}{a}{b}{c}"),
        ]
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .min()
        .unwrap();

        println!(
            "{}",
            clock_nums.iter().position(|&num| num == clock_num).unwrap() + 1
        );
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
