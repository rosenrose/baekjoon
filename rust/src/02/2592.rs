fn main() {
    let mut buf = String::new();

    let nums = parse_int_vec_lines(&mut buf, 10);

    let avg = nums.iter().sum::<i32>() / nums.len() as i32;

    let counts = nums.iter().map(|&num| {
        let count = nums.iter().filter(|&n| *n == num).count();
        (num, count)
    });

    let (mode, _) = counts.max_by_key(|&(_, c)| c).unwrap();

    println!("{avg}\n{mode}");
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
