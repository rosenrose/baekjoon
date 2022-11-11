fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for i in 1..=n {
        read_line(&mut buf);
        let s1 = buf.to_string();

        read_line(&mut buf);
        let s2 = buf.to_string();

        println!(
            "Data Set {i}: {}",
            if sanitize(&s1) == sanitize(&s2) {
                "equal"
            } else {
                "not equal"
            }
        );

        if i < n {
            println!("");
        }
    }
}

fn sanitize(input: &String) -> String {
    let mut cleaned = input.trim().to_lowercase();

    while cleaned.contains("  ") {
        cleaned = cleaned.replace("  ", " ");
    }

    cleaned = cleaned
        .replace(['[', '{'], "(")
        .replace([']', '}'], ")")
        .replace(';', ",");

    for sp in ["(", ")", ".", ",", ":"] {
        let left = format!(" {sp}");
        let right = format!("{sp} ");

        while cleaned.contains(&left) || cleaned.contains(&right) {
            cleaned = cleaned.replace(&left, sp).replace(&right, sp);
        }
    }

    cleaned
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
