use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();
    let mut substr = HashSet::with_capacity(input.len());
    let mut count = 0;

    for i in 1..=input.len() {
        for j in 0..=input.len() - i {
            substr.insert(&input[j..j + i]);
        }

        count += substr.len();
        substr.clear();
    }

    println!("{count}");
}
