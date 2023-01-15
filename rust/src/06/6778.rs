use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let (a, e) = (input.next().unwrap(), input.next().unwrap());

    if matches!((a, e), (3.., ..=4)) {
        println!("TroyMartian");
    }
    if matches!((a, e), (..=6, 2..)) {
        println!("VladSaturnian");
    }
    if matches!((a, e), (..=2, ..=3)) {
        println!("GraemeMercurian");
    }
}
