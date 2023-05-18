use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().map(str::as_bytes);

    while let (Some(a), Some(b)) = (input.next(), input.next()) {
        if (a, b) == (b"0", b"0") {
            return;
        }

        let a: Vec<_> = a.iter().rev().map(|ch| ch - b'0').collect();
        let b: Vec<_> = b.iter().rev().map(|ch| ch - b'0').collect();
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
