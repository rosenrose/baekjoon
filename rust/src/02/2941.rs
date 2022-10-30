fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let croatian_letters = ["c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z="];
    let word = buf.trim();
    let mut len = word.len();

    for letter in croatian_letters {
        let count = word.matches(letter).count();
        len -= count;
    }

    println!("{len}");
}
