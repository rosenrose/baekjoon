fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let croatian_letters = ["c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z="];
    let word = buf.trim();

    let len = croatian_letters
        .iter()
        .fold(word.len(), |len, letter| len - word.matches(letter).count());

    println!("{len}");
}
