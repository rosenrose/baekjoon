fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let result = ('a'..='z').map(
        |letter| match buf.trim().chars().position(|c| c == letter) {
            Some(i) => i as i32,
            None => -1,
        },
    );

    for r in result {
        print!("{r} ");
    }
}
