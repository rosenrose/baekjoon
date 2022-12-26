fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let year: i32 = buf.trim().parse().unwrap();

    let is_leap = if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                true
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    };

    println!("{}", if is_leap { 1 } else { 0 });
}
