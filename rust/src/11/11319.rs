use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    for input in buf.lines().skip(1).map(str::to_lowercase) {
        let c = input
            .matches(|ch| ch != ' ' && !VOWELS.contains(&ch))
            .count();
        let v = input.matches(VOWELS).count();

        println!("{c} {v}");
    }
}
