fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let record = buf.trim().as_bytes();

    let mut score = match record[0] as char {
        'A' => 4.0,
        'B' => 3.0,
        'C' => 2.0,
        'D' => 1.0,
        'F' => {
            println!("0.0");
            return;
        }
        _ => Default::default(),
    };

    score += match record[1] as char {
        '+' => 0.3,
        '-' => -0.3,
        _ => Default::default(),
    };

    println!("{score:.1}");
}
