fn main() {
    let mut buf = String::new();

    let nums = parse_int_vec_lines(&mut buf, 2);
    let multiplier = nums[1].to_string();

    let result = multiplier
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as i32 * nums[0]);

    for r in result {
        println!("{r}");
    }
    println!("{}", nums[0] * nums[1]);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec_lines(buf: &mut String, n: i32) -> Vec<i32> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf)
        })
        .collect()
}
