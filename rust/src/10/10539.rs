fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let mut sum = 0;

    buf.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .enumerate()
        .for_each(|(i, avg)| {
            let num = avg * (i as i32 + 1) - sum;
            print!("{num} ");

            sum += num;
        });
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
