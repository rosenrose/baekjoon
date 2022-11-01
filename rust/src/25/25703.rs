fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    println!("int a;");

    for i in 1..=n {
        let deref = "*".repeat(i);

        match i {
            1 => println!("int {deref}ptr = &a;"),
            2 => println!("int {deref}ptr{i} = &ptr;"),
            _ => println!("int {deref}ptr{i} = &ptr{};", i - 1),
        };
    }
}
