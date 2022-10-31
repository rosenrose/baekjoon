fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let result = ('a'..='z').map(|c| buf.matches(c).count());

    for r in result {
        print!("{r} ");
    }
}
