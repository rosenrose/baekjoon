fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let score: i32 = buf.trim().parse().unwrap();

    match score {
        90..=100 => println!("A"),
        80..=89 => println!("B"),
        70..=79 => println!("C"),
        60..=69 => println!("D"),
        _ => println!("F"),
    }
}
