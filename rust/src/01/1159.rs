use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let names: Vec<char> = parse_str_vec_lines(&mut buf, n)
        .iter()
        .map(|s| s.chars().nth(0).unwrap())
        .collect();

    let letters: HashSet<&char> = names.iter().collect();
    let mut availables: Vec<&char> = letters
        .into_iter()
        .filter(|letter| names.iter().filter(|c| c == letter).count() >= 5)
        .collect();

    if availables.len() == 0 {
        println!("PREDAJA");
        return;
    }

    availables.sort();

    println!("{}", String::from_iter(availables));
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec_lines(buf: &mut String, n: i32) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
