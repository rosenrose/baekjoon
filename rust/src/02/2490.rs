use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for _ in 0..3 {
        let fronts = (0..4).filter(|_| input.next() == Some(0)).count();
        let yut = if matches!(fronts, 1..=4) {
            ('A' as u8 + fronts as u8 - 1) as char
        } else {
            'E'
        };

        println!("{yut}");
    }
}
