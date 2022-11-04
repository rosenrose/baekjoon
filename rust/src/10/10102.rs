use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let votes = buf.trim();
    let a_count = votes.chars().filter(|&c| c == 'A').count();

    match (votes.len() - a_count).cmp(&a_count) {
        Ordering::Less => println!("A"),
        Ordering::Equal => println!("Tie"),
        Ordering::Greater => println!("B"),
    };
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
