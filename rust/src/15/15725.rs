fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if !buf.contains('x') {
        println!("0");
        return;
    }

    let coef: i32 = match buf.split('x').next().unwrap() {
        "" => 1,
        "-" => -1,
        s => s.parse().unwrap(),
    };

    println!("{coef}");
}
