fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let x = parse_int_vec(&buf)[1];
    read_line(&mut buf);

    let result = buf
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|&n| n < x);

    for r in result {
        print!("{r} ");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
