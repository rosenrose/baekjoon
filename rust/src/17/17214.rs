fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();

    if !input.contains('x') {
        match input {
            "0" => println!("W"),
            "1" => println!("x+W"),
            "-1" => println!("-x+W"),
            s => println!("{s}x+W"),
        }

        return;
    }

    let mut poly = input.split('x');

    let coef_xx = match parse_int(poly.next().unwrap()) / 2 {
        1 => "".to_owned(),
        -1 => "-".to_owned(),
        x => x.to_string(),
    };

    let coef_x = poly.next().unwrap();

    if coef_x.is_empty() {
        println!("{coef_xx}xx+W");
        return;
    }

    let coef_x = match parse_int(coef_x) {
        1 => "+".to_owned(),
        -1 => "-".to_owned(),
        x => format!("{x:+}"),
    };

    println!("{coef_xx}xx{coef_x}x+W");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
