use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    const NICK1: [&str; 6] = ["Yakk", "Doh", "Seh", "Ghar", "Bang", "Sheesh"];
    const NICK2: [&str; 6] = ["Habb Yakk", "Dobara", "Dousa", "Dorgy", "Dabash", "Dosh"];

    for (i, (a, b)) in (1..=input()).map(|i| (i, (input(), input()))) {
        print!("Case {i}: ");

        match (a.max(b), a.min(b)) {
            (6, 5) => println!("Sheesh Beesh"),
            (a, b) if a == b => println!("{}", NICK2[a - 1]),
            (a, b) => println!("{} {}", NICK1[a - 1], NICK1[b - 1]),
        }
    }
}
