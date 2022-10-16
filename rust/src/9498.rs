fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let score: i32 = buf.trim().parse().unwrap();

    match score {
        s if (90..=100).contains(&s) => println!("A"),
        s if (80..90).contains(&s) => println!("B"),
        s if (70..80).contains(&s) => println!("C"),
        s if (60..70).contains(&s) => println!("D"),
        _ => println!("F"),
    }
}
