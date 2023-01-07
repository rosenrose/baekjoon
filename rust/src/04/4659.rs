use std::io;

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
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if pwd.matches(vowels).count() == 0 {
        return false;
    }

    if pwd.split(vowels).any(|s| s.len() >= 3) {
        return false;
    }
    if pwd.split(|c| !vowels.contains(&c)).any(|s| s.len() >= 3) {
        return false;
    }

    if ('a'..='z')
        .filter_map(|c| (c != 'e' && c != 'o').then(|| c.to_string().repeat(2)))
        .any(|s| pwd.contains(&s))
    {
        return false;
    }

    true
}
