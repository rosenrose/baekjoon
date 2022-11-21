fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_str_vec(&buf)[..] {
        let (a_index, b_index) = a
            .char_indices()
            .filter_map(|(a_idx, ch)| {
                if b.contains(ch) {
                    Some((a_idx, b.find(ch).unwrap()))
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        b.char_indices().for_each(|(i, ch)| {
            if i == b_index {
                println!("{a}");
                return;
            }

            println!(
                "{}{ch}{}",
                ".".repeat(a_index),
                ".".repeat(a.len() - a_index - 1)
            );
        });
    }
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
