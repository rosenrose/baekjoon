fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let arr = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    const N: i32 = 1000000;
    let (mut min, mut max) = (N, -N);

    for num in arr {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }

    println!("{min} {max}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
