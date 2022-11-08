fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let seats = buf.trim();
    let count = seats.len();

    let mut seats: String = seats
        .split_inclusive("LL")
        .map(|s| s.split_inclusive('S'))
        .flatten()
        .map(|s| format!("*{s}"))
        .collect();

    seats.push('*');
    // println!("{seats}");

    seats = seats
        .replace("*S", "")
        .replace("*L", "")
        .replace("S*", "")
        .replace("L*", "");

    let no_holders = seats.replace('*', "").len();

    println!("{}", count - no_holders);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}