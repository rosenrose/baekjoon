fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut words = Vec::new();
    let mut tags = Vec::new();

    buf.trim().split_inclusive(['<', '>']).for_each(|s| {
        if s.ends_with('>') {
            tags.push(s.trim_end_matches('>'));
        } else {
            words.push(s.trim_end_matches('<'));
        }
    });

    if words.len() == tags.len() {
        words.push("");
    }
    // println!("{words:?} {tags:?}");
    for (i, word) in words.iter().enumerate() {
        let word: Vec<String> = word.split(' ').map(|s| s.chars().rev().collect()).collect();

        if i > 0 {
            print!("<{}>", tags[i - 1]);
        }
        print!("{}", word.join(" "));
    }
}
