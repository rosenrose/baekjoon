fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let result = ('a'..='z').map(|c| match buf.find(c) {
        Some(i) => i as i32,
        None => -1,
    });

    for r in result {
        print!("{r} ");
    }
}
