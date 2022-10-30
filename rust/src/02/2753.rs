fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let year: i32 = buf.trim().parse().unwrap();

    let is_leap = if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                1
            } else {
                0
            }
        } else {
            1
        }
    } else {
        0
    };

    println!("{is_leap}");
}
