fn main() {
    let mut buf = String::new();
    let mut min = 100;

    let sum = (0..7)
        .map(|_| {
            read_line(&mut buf);
            parse_int(&buf)
        })
        .filter(|i| i % 2 == 1)
        .fold(0, |sum, i| {
            min = i.min(min);

            sum + i
        });

    if sum == 0 {
        println!("-1");
        return;
    }

    println!("{sum}\n{min}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
