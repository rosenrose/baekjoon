fn main() {
    let mut buf = String::new();

    let class1 = parse_int_vec_lines(&mut buf, 4);
    let class2 = parse_int_vec_lines(&mut buf, 2);

    let mut max_sum = 0;

    for c1 in &class1 {
        for c2 in &class2 {
            let sum = (class1.iter().sum::<i32>() - c1) + c2;
            max_sum = sum.max(max_sum);
        }
    }

    println!("{max_sum}");
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
