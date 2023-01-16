use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for input in buf.lines().skip(1).map(|input| input.to_lowercase()) {
        let c = input
            .matches(|ch| ch != ' ' && !vowels.contains(&ch))
            .count();
        let v = input.matches(vowels).count();

        println!("{c} {v}");
    }
}
