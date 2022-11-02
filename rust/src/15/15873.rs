fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let ab = buf.trim().as_bytes();
    let first = ab[0] - '0' as u8;
    let last = ab.last().unwrap() - '0' as u8;

    let sum = match ab.len() {
        2 => first + last,
        3 => {
            if ab[1] as char == '0' {
                10 + last
            } else {
                first + 10
            }
        }
        4 => 20,
        _ => 0,
    };

    println!("{sum}");
}
