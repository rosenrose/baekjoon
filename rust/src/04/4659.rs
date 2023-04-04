use std::io;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "end") {
        println!(
            "<{input}> is {}",
            if is_acceptable(input) {
                "acceptable."
            } else {
                "not acceptable."
            }
        );
    }
}

fn is_acceptable(pwd: &str) -> bool {
    if pwd.matches(VOWELS).count() == 0 {
        return false;
    }

    if pwd.split(VOWELS).any(|s| s.len() >= 3) {
        return false;
    }
    if pwd.split(|ch| !VOWELS.contains(&ch)).any(|s| s.len() >= 3) {
        return false;
    }

    if ('a'..='z')
        .filter_map(|c| (c != 'e' && c != 'o').then_some(c.to_string().repeat(2)))
        .any(|s| pwd.contains(&s))
    {
        return false;
    }

    true
}
