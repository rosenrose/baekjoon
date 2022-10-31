fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    for i in 1..=n {
        println!("Hello World, Judge {i}!");
    }
}
