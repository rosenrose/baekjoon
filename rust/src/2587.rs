fn main() {
    let mut buf = String::new();

    let mut nums = parse_int_vec_lines(&mut buf, 5);
    nums.sort();

    println!(
        "{}\n{}",
        nums.iter().sum::<i32>() / nums.len() as i32,
        nums[2]
    );
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
