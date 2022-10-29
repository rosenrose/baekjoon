use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let (mut y, mut m) = (0, 0);
    let times = parse_int_vec(&buf);

    for time in times {
        y += ((time / 30) + 1) * 10;
        m += ((time / 60) + 1) * 15;
    }

    let (plan, cost) = match y.cmp(&m) {
        Ordering::Less => ("Y", y),
        Ordering::Equal => ("Y M", y),
        Ordering::Greater => ("M", m),
    };

    println!("{plan} {cost}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
