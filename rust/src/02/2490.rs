use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..3 {
        let fronts = (0..4).filter(|_| input.next().unwrap() == 0).count();
        let yut = if matches!(fronts, 1..=4) {
            ('A' as u8 + fronts as u8 - 1) as char
        } else {
            'E'
        };

        println!("{yut}");
    }
}
