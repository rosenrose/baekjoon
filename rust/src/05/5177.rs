use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();
    let n: i32 = input.next().unwrap().parse().unwrap();

    for i in 1..=n {
        println!(
            "Data Set {i}: {}",
            if sanitize(input.next().unwrap()) == sanitize(input.next().unwrap()) {
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

fn sanitize(input: &str) -> String {
    let mut cleaned = input.to_lowercase();

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
