fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let result = ('a'..='z').map(|letter| buf.trim().chars().filter(|&c| c == letter).count());

    for r in result {
        print!("{r} ");
    }
}
