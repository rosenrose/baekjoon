use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().enumerate().map(|(i, s)| {
        (if i == 0 {
            s.parse()
        } else {
            i128::from_str_radix(s, 2)
        })
        .unwrap()
    });

    for _ in 0..input.next().unwrap() {
        println!("{:b}", input.next().unwrap() + input.next().unwrap());
    }
}
