fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let result = ('a'..='z').map(|c| match buf.match_indices(c).next() {
        Some((i, _)) => i as i32,
        None => -1,
    });

    for r in result {
        print!("{r} ");
    }
}
