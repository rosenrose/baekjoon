use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let b: i32 = buf.trim().parse().unwrap();
    let p = 5 * b - 400;

    println!(
        "{p}\n{}",
        match p.cmp(&100) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    );
}
