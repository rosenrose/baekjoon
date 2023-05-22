fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    const COMPARE: &str = "UCPC";
    let comapre_chars: Vec<_> = COMPARE.chars().collect();

    let mut stack = String::new();

    for ch in buf.trim().chars() {
        if ch == comapre_chars[stack.len()] {
            stack.push(ch);
        }

        if stack == COMPARE {
            println!("I love UCPC");
            return;
        }

        if stack.len() >= COMPARE.len() {
            break;
        }
    }

    println!("I hate UCPC");
}
