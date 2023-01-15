fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut clock_nums = Vec::new();

    for a in 1..=9 {
        for b in 1..=9 {
            for c in 1..=9 {
                for d in 1..=9 {
                    clock_nums.push(get_clock_num(a, b, c, d));
                }
            }
        }
    }

    clock_nums.sort();
    clock_nums.dedup();
    // println!("{clock_nums:?} {}", clock_nums.len());
    let [a, b, c, d] = parse_int_vec(&buf)[..] else { return };
    let clock_num = get_clock_num(a, b, c, d);

    println!(
        "{}",
        clock_nums.iter().position(|&num| num == clock_num).unwrap() + 1
    );
}

fn get_clock_num(a: i32, b: i32, c: i32, d: i32) -> i32 {
    [
        format!("{a}{b}{c}{d}"),
        format!("{b}{c}{d}{a}"),
        format!("{c}{d}{a}{b}"),
        format!("{d}{a}{b}{c}"),
    ]
    .iter()
    .map(|s| s.parse().unwrap())
    .min()
    .unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
