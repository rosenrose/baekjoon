use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    while let (Some(a), Some(b)) = (input.next(), input.next()) {
        if (a, b) == ("0", "0") {
            return;
        }

        let a: Vec<_> = a.chars().rev().map(|ch| ch as u8 - '0' as u8).collect();
        let b: Vec<_> = b.chars().rev().map(|ch| ch as u8 - '0' as u8).collect();
        let mut count = 0;
        let mut carry = 0;

        for i in 0..a.len().max(b.len()) {
            let num = carry + a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0);

            if num >= 10 {
                count += 1;
            }

            carry = num / 10;
        }

        if carry >= 10 {
            count += 1;
        }

        println!("{count}");
    }
}
