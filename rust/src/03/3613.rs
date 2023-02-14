enum Format {
    Cpp,
    Java,
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let name = buf.trim();
    const ERROR: &str = "Error!";

    if name.chars().all(char::is_lowercase) {
        println!("{name}");
        return;
    }

    if name.chars().nth(0).unwrap().is_uppercase() {
        println!("{ERROR}");
        return;
    };

    let format = if name.contains('_') {
        if name.starts_with('_') || name.ends_with('_') {
            println!("{ERROR}");
            return;
        }
        if name.split('_').any(str::is_empty) {
            println!("{ERROR}");
            return;
        }
        if name.chars().any(char::is_uppercase) {
            println!("{ERROR}");
            return;
        }

        Format::Cpp
    } else {
        Format::Java
    };

    let converted: String = match format {
        Format::Cpp => name
            .split('_')
            .enumerate()
            .map(|(i, s)| {
                if i == 0 {
                    s.to_owned()
                } else {
                    format!("{}{}", s.chars().nth(0).unwrap().to_uppercase(), &s[1..])
                }
            })
            .collect(),
        Format::Java => name
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_string()
                }
            })
            .collect(),
    };

    println!("{converted}");
}
