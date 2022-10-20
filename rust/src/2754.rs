fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut score = 0.0;
    let mut record = buf.trim().chars();

    match record.next().unwrap() {
        'A' => score = 4.0,
        'B' => score = 3.0,
        'C' => score = 2.0,
        'D' => score = 1.0,
        'F' => {
            println!("0.0");
            return;
        }
        _ => (),
    };

    match record.next().unwrap() {
        '+' => score += 0.3,
        '-' => score -= 0.3,
        _ => (),
    }

    println!("{score:.1}");
}
