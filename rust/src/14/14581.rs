fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let id = buf.trim();
    let fan = ":fan:";

    println!("{}", fan.repeat(3));
    println!("{fan}:{id}:{fan}");
    println!("{}", fan.repeat(3));
}
