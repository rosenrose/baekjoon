fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let m = parse_int(&buf);
    read_line(&mut buf);

    let n = parse_int(&buf);

    let square_nums: Vec<_> = (1..)
        .skip_while(|i| i * i < m)
        .take_while(|i| i * i <= n)
        .map(|i| i * i)
        .collect();

    if square_nums.len() == 0 {
        println!("-1");
        return;
    }

    println!(
        "{}\n{}",
        square_nums.iter().sum::<i32>(),
        square_nums.iter().min().unwrap()
    );
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
