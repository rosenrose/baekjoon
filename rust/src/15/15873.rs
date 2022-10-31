fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let ab = buf.trim();
    let first = ab.chars().nth(0).unwrap().to_digit(10).unwrap();
    let last = ab.chars().last().unwrap().to_digit(10).unwrap();

    let sum = match ab.len() {
        2 => first + last,
        3 => {
            if ab.chars().nth(1).unwrap() == '0' {
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
