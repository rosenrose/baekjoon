fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();
    let Some((coef_x, constant)) = input.split_once('x') else {
        match input {
            "0" => println!("W"),
            "1" => println!("x+W"),
            "-1" => println!("-x+W"),
            c => println!("{c}x+W"),
        }
        return;
    };

    let coef_xx = match parse_int(coef_x) / 2 {
        1 => "".to_owned(),
        -1 => "-".to_owned(),
        x => x.to_string(),
    };

    if constant.is_empty() {
        println!("{coef_xx}xx+W");
        return;
    }

    let coef_x = match parse_int(constant) {
        1 => "+".to_owned(),
        -1 => "-".to_owned(),
        x => format!("{x:+}"),
    };

    println!("{coef_xx}xx{coef_x}x+W");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
