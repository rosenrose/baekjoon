fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let name = buf.trim();

    if name.chars().all(char::is_lowercase) {
        println!("{name}");
        return;
    }

    let first = name.chars().nth(0).unwrap();

    if first.is_uppercase() {
        println!("Error!");
        return;
    };

    let format = if name.contains('_') {
        let last = name.chars().last().unwrap();

        if first == '_' || last == '_' {
            println!("Error!");
            return;
        }
        if name.split('_').any(str::is_empty) {
            println!("Error!");
            return;
        }
        if name.chars().any(char::is_uppercase) {
            println!("Error!");
            return;
        }

        "cpp"
    } else {
        "java"
    };

    let converted: String = match format {
        "cpp" => name
            .split('_')
            .enumerate()
            .map(|(i, s)| {
                if i == 0 {
                    s.to_string()
                } else {
                    format!("{}{}", s.chars().nth(0).unwrap().to_uppercase(), &s[1..])
                }
            })
            .collect(),
        "java" => name
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_string()
                }
            })
            .collect(),
        _ => String::new(),
    };

    println!("{converted}");
}
